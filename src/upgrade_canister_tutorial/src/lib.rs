use std::cell::RefCell;

use candid::{
    utils::{ArgumentDecoder, ArgumentEncoder},
    CandidType, Deserialize,
};
use ic_cdk::{export::serde::Serialize, println};
use ic_cdk_macros::{post_upgrade, pre_upgrade};

const VERSION: &str = "0.4";

#[derive(Debug, Default, Clone, CandidType, Serialize, Deserialize)]
pub struct Student {
    name: String,
    age: u16,
}

impl Student {
    fn new(name: String, age: u16) -> Self {
        Self { name, age }
    }
}

impl Student {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_age(&mut self, age: u16) {
        self.age = age;
    }
}

impl ArgumentEncoder for Student {
    fn encode(self, ser: &mut candid::ser::IDLBuilder) -> candid::Result<()> {
        println!("encode for Student {}", VERSION);
        ser.arg(&self.name)?;
        ser.arg(&self.age)?;
        Ok(())
    }
}

impl<'de> ArgumentDecoder<'de> for Student {
    fn decode(de: &mut candid::de::IDLDeserialize<'de>) -> candid::Result<Self> {
        println!("decode for Student {}", VERSION);
        Ok(Student {
            name: de.get_value()?,
            age: de.get_value().unwrap_or_default(),
        })
    }
}

pub struct MaybeStable(Vec<Student>);

impl ArgumentEncoder for MaybeStable {
    fn encode(self, ser: &mut candid::ser::IDLBuilder) -> candid::Result<()> {
        println!("encode for MaybeStable {}", VERSION);
        self.0.into_iter().map(|s| s.encode(ser)).collect()
    }
}

impl<'de> ArgumentDecoder<'de> for MaybeStable {
    fn decode(de: &mut candid::de::IDLDeserialize<'de>) -> candid::Result<Self> {
        println!("decode for MaybeStable {}", VERSION);
        let mut v = vec![];
        while let Ok(s) = Student::decode(de) {
            v.push(s);
        }
        Ok(MaybeStable(v))
    }
}

thread_local! {
    static STUDENT: RefCell<Student> = RefCell::new(Student::default());
    static CLASS: RefCell<Vec<Student>> = RefCell::new(Vec::new());
}

#[ic_cdk_macros::query]
fn greet() -> String {
    CLASS.with(|cs| format!("Hello: {:?}", cs.borrow()))
}

#[ic_cdk_macros::update]
fn set_name(name: String) {
    STUDENT.with(|s| s.borrow_mut().set_name(name))
}

#[ic_cdk_macros::update]
fn set_age(age: u16) {
    STUDENT.with(|s| s.borrow_mut().set_age(age))
}

#[ic_cdk_macros::update]
fn new_student(name: String, age: u16) {
    CLASS.with(|class| class.borrow_mut().push(Student::new(name, age)))
}

#[pre_upgrade]
fn pre_upgrade() {
    println!("pre version {}", VERSION);

    // let ms = MaybeStable(STUDENT.with(|s| s.borrow().clone()));
    let ms = MaybeStable(CLASS.with(|class| class.borrow().clone()));
    ic_cdk::storage::stable_save(ms).expect("stable save")
}

#[post_upgrade]
fn post_upgrade() {
    println!("post version {}", VERSION);

    let ms = ic_cdk::storage::stable_restore::<MaybeStable>().expect("stable restore");
    CLASS.with(|class| *class.borrow_mut() = ms.0);
    // STUDENT.with(|s| *s.borrow_mut() = ms.0);
}
