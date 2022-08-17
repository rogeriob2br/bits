#![no_std]
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::types::List;

mod types;
#[derive(Copy, Clone)]
pub struct Pid{
    i:u8,
    b:u8
}
impl Pid{
    pub fn new(i:u8, b:u8)->Pid{
        Pid{
            i,
            b
        }
    }

    pub fn get_i(&self) -> u8 {
        self.i
    }
    pub fn get_b(&self) -> u8 {
        self.b
    }
    pub fn set_i(&mut self, i: u8) {
        self.i = i;
    }
    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }
}

#[derive()]
pub struct Bitmap {
    bitset: Vec<u8>,
    next_pid: Pid,
    base: u8,
    last_free_pids: List<Pid>
}

impl Bitmap {
    pub fn new(pid_max_limit:usize) ->Bitmap{
        let mut bitset: Vec<u8> = Vec::with_capacity(pid_max_limit);
        for i in 0..127 {
            bitset.push(0);
        }
        let mut next_pid:Pid = Pid::new(0,0);
        let base:u8 = 2;
        let last_free_pids:List<Pid> = List::new();
        Bitmap{ bitset, next_pid, base , last_free_pids}
    }
    pub fn get_next_pid(&self)->Pid{
        self.next_pid.clone()
    }

    pub fn set_next_pid(&mut self){
        let base: u8= 2;
        let pid = self.next_pid.clone();
        let b = pid.get_b();

        if b >= 8{
            let i = pid.get_i();
            let len = self.bitset.len();
            if i < len as u8 {
                let new_pid = Pid::new(i+1, 0);
            }
            else{
                //TODO: Error Handling
            }
        }else{
            let exp = pid.get_b()+1;
            let pow = base.pow(exp as u32);

        }



    }
    pub fn set_bit_on(&mut self , pid:Pid){
        let base: u8= 2;
        let exp = base.pow(pid.get_b() as u32);
        self.bitset[pid.get_i() as usize] += exp;
    }
    pub fn alloc_pid(&mut self) ->Pid{

        // TODO: Error handling
        // TODO: Make more Generic
        // TODO: Make more funcional
        let node = self.last_free_pids.pop();
        if node.is_some(){
            let base: u8= 2;
            let pid = self.last_free_pids.pop();
            let exp = base.pow(pid.get_b() as u32);
            self.bitset[pid.get_i()] += exp;
            return pid;
        }

        //TODO: Check if exist a pid to kill
        // TODO: Panic if dont have any pids to alloc
        else if self.bitset.len() > self.next_pid.get_i(){
            if self.next_pid.get_b()<=8{
                let pid = self.next_pid.clone();
                // TODO: Set new next_bit
                self.set_bit_on(pid.clone());
                return  pid;
            }
            else if self.bitset.len() > self.next_pid.get_i()+1{
                //TODO: Set next bit to next i position
            }

        }

        //TODO: Make handling structure
        return  self.next_pid.clone()


    }

    pub fn dealloc_pid(&mut self, pid: Pid) {
        //TODO: Insert in free pids
        //TODO: Set bit to zero
    }

}

fn check_bit_match(num:u8, b: u8)->bool{
    let base: u8= 2;
    let mut exp = base.pow((b.clone()+1) as u32);

    let mut rest = num.clone()%exp.clone();

    if rest.clone() > ((exp.clone()/2)-1){
        true
    }
    else{
        false
    }
}
fn fint_next_bit_empty_in_bit_map(bitmap:Bitmap, pid: Pid)->Pid{

    for i in pid.i.clone()..127 {
        let num = bitmap.bitset[i];
        let bit =find_bit_empty_in_num(num);


    }
    pid
}
fn find_bit_empty_in_num(num:u8)->u8{
    let mut attempt: bool;
    let mut i: u8 = 0;
    while attempt.clone() && i.clone()<=7 {
        attempt = check_bit_match(num,i);
        if attempt.clone() {
            i += 1;
        }
    }
    i
}




//#![no_main]
fn main() {
    //println!("Hello, world!");
    let mut bitmap: Vec<u8> = Vec::with_capacity(128);

    let mut a:u8 = 2;
    let mut b = a.pow(1);

    for i in 0..127 {
        bitmap.push(0);
    }
    bitmap[0] = 27;
    let bit1:u8 = 3;
    if check_bit_match(bitmap[0].clone(), bit1){
         //println!("bit 1 ligado {} - {}", bitmap[0].clone()%2, bitmap[0].clone())
     }else {
         //println!("bit 1 desligado {} - {}", bitmap[0].clone()%2, bitmap[0].clone())
    }

}


