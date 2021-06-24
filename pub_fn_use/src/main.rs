mod my_mod{
    fn private_fn() {
        println!("called `my_mod::private_fn`");
    }
    
    pub fn function(){
        println!("called `my_mod::function`");
    }

    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n");
        private_fn();
    }
    
    pub mod nested {
        pub fn function(){
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // This pub fn belongs to my_mod module
        
        pub (in crate::my_mod) fn public_function_in_my_mod() {
            print!("called my_mod::nested::public_fucntion_in_my_mod");
        }

        pub(self) fn public_function_in_nested_() {
           println!("called `my_mod::nested::public_function_in_nested()`");
        }

        pub(super) fn public_function_in_super_mod() {
            print!("called `my_mod::nested::public_function_in_super_mod()`");
        }

    }

    pub fn call_public_function_in_my_mod() {
    }
}
