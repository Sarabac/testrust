use std::path::Path;

macro_rules! filetest  {
    (std::path::Path::new($name: literal).exists()) => {
        if std::path::Path::new($name).exists(){
            "whaaa".to_string()
        }else{
            "bheuu".to_string()
        }
        
    };
}


#[cfg(test)]
mod test {
    #[test]
    fn hi() {
        println!("hello");
        let fi = filetest!(std::path::Path::new("ert").exists());
        dbg!(fi);
    }
}