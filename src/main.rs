use std::env;
mod bfi;

fn main() {
    // print "MiKoronjoo"
    bfi::run_code("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>+++++++.>+++++.<--.>++++++.+++.---.-.----.+++++..<<<.".to_string());

    // print the reversed user input
    bfi::run_code("---------[++++++++++>,----------]<-[+.<-]++++++++++.".to_string());

    // run a code from a file
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        bfi::run_file(&args[1]);
    }
}
