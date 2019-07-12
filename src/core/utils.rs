pub fn print_title(title : &str){
    print_start_line();
    print_content(title);
    print_end_line();
}

fn print_start_line(){
    println!("********************************");
}

fn print_content(content : &str){
    println!("         {}         ",content);
}

fn print_end_line(){
    println!("********************************");
}