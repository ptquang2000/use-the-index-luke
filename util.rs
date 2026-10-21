#[path = "util_query.rs"]
mod query;
use self::query::*;

pub use std::io::Write;

pub struct UtilMemory {
    pub db: *const (),
    pub query: Option<fn(*const (), *const u8)>,
}

impl UtilMemory {
    fn query(&self, q: *const u8) {
        if let Some(func) = self.query {
            func(self.db, q);
        }
    }
}

pub fn create_table_with_1000_rows(util_memory: &UtilMemory) {
    util_memory.query(EMPLOYEES_TABLE_WITH_1000_ENTRIES.as_ptr());
}

pub fn merge_with_very_big_company(util_memory: &UtilMemory) {
    util_memory.query(MERGER_WITH_VERY_BIG_COMPANY.as_ptr());
    util_memory.query(
        "
CREATE UNIQUE INDEX employees_pk
    ON employees (employee_id, subsidiary_id);
            \0"
        .as_ptr(),
    );
}

pub fn select_employee_by_id(util_memory: &UtilMemory, execution_plan: bool, employee_id: i32) {
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
    util_memory.query(buffer.as_ptr());
}
