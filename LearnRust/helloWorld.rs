mod utils;

fn main() {
    let s1 = State::Uninitialized;
    println!("{}", show_state(s1));

    let s2 = State::Error("Hello World!".to_string());
    println!("{}", show_state(s2));

    println!("{}", life_time_play());
}

/*
enum String1<'a> {
    String(String),
    Str(&'a str),
}

fn to_string(s: String1) -> String {
    return match s {
        String1::String(s1) => s1,
        String1::Str(s1) => s1.to_string(),
    };
}

fn to_string1<T>(s: T) -> String1{
    return match s1 {
        String => String1::String(s1),
        &str => String1::Str(s1),
    }
}
*/

trait GetString {
    fn get_string(self) -> String;
}

impl GetString for String { fn get_string(self) -> String { self }}
impl GetString for &str { fn get_string(self) -> String { (self).to_string() }}

//fn to_string<T> (s: T) -> String {
//    return match TypeId::of::<T>() {
//        TypeId::of::<String> => s,
//        TypeId::of::<&str> => s.to_string(),
//        _ => "".to_string(),
//    };
//}

enum State {
    Uninitialized,
    Initialized,
    Waiting,
    Done(String),
    Error(String),
}

fn show_state(state: State) -> String {
    let s = match state {
        State::Uninitialized => "Uninitialized".get_string(),
        State::Initialized => "Initialized".get_string(),
        State::Waiting => "Waiting for user input".get_string(),
        State::Done(s) => format!("Done with message {}", s).get_string(),
        State::Error(s) => format!("Failed with error {}", s).get_string(),
    };
    return s;
}

fn life_time_play<'a>() -> &'a str {
    let s = "This string survives";
    //let x = "This string".to_string();
    //let z = &'a x;
    //return z;
    return s;
}