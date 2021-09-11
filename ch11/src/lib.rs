/***** CH11_1 Problem
1. Complete panic_when_num_is_bigger in a form that the test can be perfomed.
2. Make the panic_case to pass the test     ( add  "should_panic expect ) 
*****/
pub fn panic_when_num_is_bigger()
{
               if num >num2{
                        panic!("{} is bigger than {}", num, num2);
                }
}

#[cfg(test)]
mod tests
{
	#[test]
        fn panic_case(){
		// do not edit
                panic_when_num_is_bigger(10, 11);
                panic_when_num_is_bigger(0.5, 0.9);
                panic_when_num_is_bigger(-5, -11);
        }			
}










/******************************************************************************
Answer
pub fn panic_when_num_is_bigger<T: PartialOrd+std::fmt::Display>(num: T, num2 :T )
{
               if num >num2{
                        panic!("{} is bigger than {}", num, num2);
                }
}
#[cfg(test)]
mod tests
{
	use super::*;
	#[test]
        #[should_panic(expected = "-5 is bigger than -11")]
        fn panic_case(){
                panic_when_num_is_bigger(10, 11);
                panic_when_num_is_bigger(0.5, 0.9);
                panic_when_num_is_bigger(-5, -11);
        }			
}




*******************************************************************************/
