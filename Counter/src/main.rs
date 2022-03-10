pub struct Counter {
    val: i8,
}

impl Counter {
    // this is public function 
    // -> means it returns TYPE
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    //&mut means we will mutate the value
    pub fn increment(&mut self) {
        self.val = self.val + 1;
        let log_message = format!("Increase number to {}", self.val);
        println!("{}", log_message.to_string());
    }

    pub fn decrement(&mut self) {
        self.val = self.val - 1;
        let log_message = format!("Increase number to {}", self.val);
        println!("{}", log_message.to_string());
    }
}

fn main() {
    // create variable with let
    // it's a mutable var
    // initialize the Counter with value of 0 to start
    let mut counter = Counter {
        val: 0,
    };

    counter.increment();
    println!("After incrementing: {}", counter.get_num());
}