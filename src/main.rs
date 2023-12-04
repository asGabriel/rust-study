fn main() {
    trying_array();
    println!("Is weekday? {}", check_week_day(Weekday::Friday));
    vectors();
    account_test();
}

fn trying_array() {
    let notes: [f32; 4] = [10f32, 8f32, 9.5, 6.0];
    // let array2 = [5; 10];

    let int: usize = 0;
    println!("{}", notes[int]);

    for indice in 0..notes.len() {
        println!("the note {} is = {} ", indice + 1, notes[indice])
    }
}

#[allow(dead_code)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday, 
    Thursday,
    Friday,
    Saturday,
    Sunday
}

fn check_week_day(day_of_week: Weekday) -> bool {
    match day_of_week {
        Weekday::Sunday | Weekday::Saturday => true,
        _ => false
    }
}

fn vectors() {
    // let mut notes: Vec<f32> = Vec::new();
    let mut notes: Vec<f32> = Vec::with_capacity(4);
    // let mut notes: Vec<f32> = vec![0.4, 6.5, 9.0];
    println!("Capacity: {}", notes.capacity());
    
    notes.push(6.5);
    notes.push(8.0);
    notes.push(10.0);
    notes.push(9.4);
    println!("Capacity after push: {}", notes.capacity());

    // while let Some(note) = notes.pop() {
    //     println!("Removed note: {}", note);
    // }
    for note in &notes {
        println!("Note: {}", note);
    }
    println!("{:?}", notes)
}

struct Account {
    owner: Owner,
    balance: f64
}

impl Account {
    fn withdraw(&mut self, value: f64) {
        self.balance -= value;
    }
}

struct Owner {
    name: String,
    age: u8
}

fn account_test() {
    let mut acc:Account = Account {
        owner: Owner { name: (String::from("Gabriel")), age: (28) },
        balance: 100.0
    };

    acc.withdraw(50.0);

    println!("Account data: Owner = {}, Balance = {}", acc.owner.name, acc.balance);
}