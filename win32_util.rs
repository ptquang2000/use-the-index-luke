#[path = "util_query.rs"]
mod query;
use query::*;

use std::io::Write;

#[repr(C)]
struct sqlite3 {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}
type Callback = unsafe extern "system" fn(
    *mut std::ffi::c_void,
    std::ffi::c_int,
    *mut *mut std::ffi::c_char,
    *mut *mut std::ffi::c_char,
) -> std::ffi::c_int;
#[link(name = "winsqlite3")]
unsafe extern "system" {
    fn sqlite3_open(filename: *const std::ffi::c_char, ppDb: *mut *mut sqlite3) -> std::ffi::c_int;
    fn sqlite3_exec(
        db: *mut sqlite3,
        sql: *const std::ffi::c_char,
        callback: Option<Callback>,
        first_argument: *mut std::ffi::c_void,
        errmsg: *mut *mut std::ffi::c_char,
    ) -> std::ffi::c_int;
    fn sqlite3_close(db: *mut sqlite3) -> std::ffi::c_int;
    fn sqlite3_errmsg(db: *mut sqlite3) -> *mut std::ffi::c_char;
    fn sqlite3_db_config(db: *mut sqlite3, op: std::ffi::c_int, ...) -> std::ffi::c_int;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn OutputDebugStringA(lpOutputString: *const std::ffi::c_char);
}

unsafe extern "system" fn callback(
    _: *mut std::ffi::c_void,
    argc: i32,
    argv: *mut *mut i8,
    az_col_name: *mut *mut i8,
) -> i32 {
    let colums = std::slice::from_raw_parts_mut(az_col_name, argc as usize);
    let rows = std::slice::from_raw_parts_mut(argv, argc as usize);
    for (&column, &row) in colums.iter().zip(rows.iter()) {
        let mut buffer = [0u8; 256];
        let mut temp = &mut buffer as &mut [u8];
        write!(
            temp,
            "{} = {}\n",
            std::ffi::CStr::from_ptr(column).to_string_lossy(),
            if !row.is_null() {
                std::ffi::CStr::from_ptr(row).to_string_lossy()
            } else {
                std::borrow::Cow::Borrowed("NULL")
            }
        )
        .expect("Failed to write err_msg.");
        OutputDebugStringA(buffer.as_ptr() as *const _);
    }
    0
}

fn create_table_with_1000_rows(database: *mut sqlite3) {
    let mut err_msg = std::ptr::null_mut();
    unsafe {
        if sqlite3_exec(
            database,
            EMPLOYEES_TABLE_WITH_1000_ENTRIES.as_ptr() as *const _,
            Some(callback),
            std::ptr::null_mut(),
            std::ptr::addr_of_mut!(err_msg),
        ) == 0
        {
        } else {
            let mut buffer = [0u8; 256];
            let mut temp = &mut buffer as &mut [u8];
            write!(
                temp,
                "{}\n",
                std::ffi::CStr::from_ptr(err_msg).to_string_lossy()
            )
            .expect("Failed to write err_msg.");
            OutputDebugStringA(buffer.as_ptr() as *const _);
        }
    }
}

fn select_employee_by_id(database: *mut sqlite3, execution_plan: bool, employee_id: i32) {
    let mut buffer = [0u8; 256];
    let mut temp = &mut buffer as &mut [u8];
    write!(
        temp,
        "
{}
SELECT first_name, last_name
  FROM employees
 WHERE employee_id = {}\0",
        if execution_plan {
            "EXPLAIN QUERY PLAN"
        } else {
            ""
        },
        employee_id
    )
    .expect("Failed to write buffer.");
    let mut err_msg = std::ptr::null_mut();
    unsafe {
        if sqlite3_exec(
            database,
            buffer.as_ptr() as *const _,
            Some(callback),
            std::ptr::null_mut(),
            std::ptr::addr_of_mut!(err_msg),
        ) == 0
        {
        } else {
            OutputDebugStringA(err_msg);
        }
    }
}

fn reset_database(database: *mut sqlite3) {
    const SQLITE_DBCONFIG_RESET_DATABASE: i32 = 1009;
    unsafe {
        let mut err_msg = std::ptr::null_mut();
        sqlite3_db_config(database, SQLITE_DBCONFIG_RESET_DATABASE, 1, 0);
        if sqlite3_exec(
            database,
            "VACUUM\0".as_ptr() as *const _,
            None,
            std::ptr::null_mut(),
            std::ptr::addr_of_mut!(err_msg),
        ) == 0
        {
        } else {
            OutputDebugStringA(err_msg);
        }
        sqlite3_db_config(database, SQLITE_DBCONFIG_RESET_DATABASE, 0, 0);
    }
}

fn main() {
    unsafe {
        let mut database = std::ptr::null_mut();
        if sqlite3_open(
            "util.db\0".as_ptr() as *const _,
            std::ptr::addr_of_mut!(database),
        ) == 0
        {
            reset_database(database);
            create_table_with_1000_rows(database);

            select_employee_by_id(database, true, 123);
            select_employee_by_id(database, false, 123);

            sqlite3_close(database);
        } else {
            OutputDebugStringA(sqlite3_errmsg(database));
        }
    }
}
