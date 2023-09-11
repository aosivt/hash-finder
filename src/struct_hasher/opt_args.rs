use std::{env, collections::HashMap};

use log::info;

use super::error_hasher::WrongArgs;


const NAME_N: &str = "-N"; 
const NAME_F: &str = "-F"; 

#[derive(Debug, Clone, Copy)]
pub struct OptArgsHasher{
    pub count_zero: i32,
    pub count_result: i32
}

impl OptArgsHasher {
    pub fn init() -> OptArgsHasher {
        OptArgsHasher::val()
    }
    pub fn count_result(self:&Self) -> &i32 {
        &self.count_zero
    }

    pub fn val() -> OptArgsHasher {
        
        let env_args: Vec<String> = env::args().skip(1).collect();
     
        if env_args.len() != 4 {
            OptArgsHasher::init_error();
        }
        let _val = OptArgsHasher::_val(env_args);
     
        if !OptArgsHasher::_val_check_contains(&_val){
            OptArgsHasher::init_error();
        }

        OptArgsHasher::_val_generate_opt(_val)
    }
    fn _val(enume: Vec<String>) -> HashMap<String, i32>{
        let mut args: HashMap<String, i32> = HashMap::with_capacity(2);
        for (ind, arg) in enume.iter().enumerate()  {
            if ind % 2 == 0 {
                let key = arg;
                let value = enume[ind + 1].clone();
                info!("{} принимает значение {}", key, value);
                args.insert(key.clone(), value.parse::<i32>().unwrap());
            }
        }
        args
    }
    fn _val_check_contains(args: &HashMap<String, i32>) -> bool{
        args.contains_key(NAME_N) && args.contains_key(NAME_F)
    }
    fn _val_generate_opt(args: HashMap<String, i32>) -> OptArgsHasher{
        OptArgsHasher { count_zero: args.get(NAME_N).expect("").to_owned(), count_result: args.get(NAME_F).expect("").to_owned() }
    }
    fn init_error(){
        let empty = WrongArgs::of();
        let result = Err::<(),WrongArgs>(empty);
        match result {
            Err(e) => info!("{}", e),
            _ => println!("No error"),
        }
    }
    
    
}

#[test]
fn _val_test(){
    let enume = vec!["test1".to_string(),"123".to_string(),"test2".to_string(),"234".to_string()];
    let _val = OptArgsHasher::_val(enume);
    if !OptArgsHasher::_val_check_contains(&_val){
        OptArgsHasher::init_error();
    }
}
#[test]
fn _val_exist_test(){
    let enume = vec!["-N".to_string(),"123".to_string(),"-F".to_string(),"234".to_string()];
    let _val = OptArgsHasher::_val(enume);
    if !OptArgsHasher::_val_check_contains(&_val){
        OptArgsHasher::init_error();
    }
    let result = OptArgsHasher::_val_generate_opt(_val);
    assert_eq!(123,result.count_zero);
    assert_eq!(234,result.count_result);

}