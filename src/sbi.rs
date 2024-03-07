
/*
This function is used to print a string to the console.
*/
pub fn console_putchar (c : usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

/*
This function is used to shutdown the system. 
It takes a failure_code as an argument. 
If the failure_code is 0, it means the system is shutting down normally. 
Otherwise, it means the system is shutting down due to a failure. 
*/
pub fn shutdown (failure_code : usize) -> ! {
    #[allow(deprecated)]
    use sbi_rt::{system_reset,NoReason,Shutdown,SystemFailure};
    match failure_code {
        0 => system_reset(Shutdown,NoReason),
        _ => system_reset(Shutdown,SystemFailure)
    };
    unreachable!()
}