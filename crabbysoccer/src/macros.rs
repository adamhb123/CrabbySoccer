// Stateful format - pseudo-macro
pub struct StatefulFormat<'a> {
    format_str: &'a str,
    values: Vec<&'a str>
}
impl<'a> StatefulFormat<'a> {
    fn _fix_values(mut self) -> Self {
        let _expected_value_count: usize = self.format_str.matches("{}").count();
        let diff: i32 = _expected_value_count as i32 - self.values.len() as i32;
        if diff >= 0 { for i in 0..diff { self.values.push(""); }}
        else { for i in 0..diff.abs() { self.values.pop(); } };
        self
    }
    pub fn new(format_str: &'a str, values: Option<Vec<&'a str>>) -> Self {
        let _expected_value_count: usize = format_str.matches("{}").count();
        let values = match values {
            Some(mut v) => v,
            None => (0.._expected_value_count).map(|e| "").collect()
        };
        let sf = StatefulFormat { format_str, values: values };
        sf._fix_values()
    }
    pub fn get(&self) -> String {
        let mut idx = 0;
        todo!();
        //let formatted: Option<String> = self.format_str.split("{}").reduce(|a,b| format!("{}{}{}", a, &self.values[idx], b)).map(String::from);
        //formatted.unwrap()
    }
    pub fn insert(&mut self, index: usize, element: &'a str){
        self.values.insert(index, element)
    }
    pub fn assign(&mut self, values: Vec<&'a str>){
        self.values = values;
    }
}
impl std::fmt::Display for StatefulFormat<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get().as_str())
    }
}
#[macro_export]
macro_rules! stateful_format {
    ($($arg:tt)*) => {{
        let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
        StatefulFormat { , vec![$($arg:tt)+] }
    }}
}

