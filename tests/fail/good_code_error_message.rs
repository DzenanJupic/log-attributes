#[log_attributes::log(info, "{fn} was executed")]
fn good_code_error_message() -> u32 {
    let _: () = 2;
}

fn main() {}
