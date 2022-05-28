mod bubble;
mod get_int;
use crate::bubble::*;
use crate::get_int::*;
use colourizer::StyledString;
use std::io::Write;

fn main() {
    let mut amount_of_data: i32 = 0;

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //Gets length vector needs to be
    print!("Amount of data points: ");
    std::io::stdout().flush().expect("Failed to flush standard output.");

    get_int(&mut amount_of_data);
    let amount_of_data_usize: usize = amount_of_data as usize;
    let mut data_points = vec![0; 1];
    data_points.resize(amount_of_data_usize, 0);
    //Gets length vector needs to be
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //Repeatedly gets data until vec is filled
    for i in 0..amount_of_data {
        let i_usize = i as usize;
        println!("Data point {}", i_usize + 1);
        let mut input_num: i32 = 0;
        get_int(&mut input_num);
        data_points[i_usize] = input_num;
    }
    //Repeatedly gets data until vec is filled
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    println!("{}", "Unsorted Data: ".rgb("0;60;255"));

    for i in 0..amount_of_data {
        let i_usize = i as usize;
        if i == amount_of_data - 1 {
            println!("{}", data_points[i_usize]);
        }
        else {
            print!("{}, ", data_points[i_usize]);
        }
    }

    bubble_sort(&mut data_points);

    println!("{}", "Sorted Data: ".rgb("0;255;50"));

    for i in 0..amount_of_data {
        let i_usize = i as usize;
        if i == amount_of_data - 1 {
            println!("{}", data_points[i_usize]);
        }
        else {
            print!("{}, ", data_points[i_usize]);
        }
    }
}
