    pub fn fibbo(num: i32) -> i32 {
      let mut previous_number: i32 = 1;
      let mut current_number: i32 = 0;
        
      let mut  i = 0;
      while i < num {
          let previous_previous_number = previous_number;
          previous_number = current_number + &previous_previous_number;
          current_number = previous_previous_number;
          i += 1;
      }
      current_number.into()
    }
//}
