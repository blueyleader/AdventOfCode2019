use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

use num::integer::lcm;

fn main() {

    // let history = HashMap::new();
    let mut history = HashSet::new();

    let mut positions = vec![vec![7,10,17], vec![-2,7,0], vec![12,5,12], vec![5,-8,6]]; //real inputs
    // let mut positions = vec![vec![-1,0,2], vec![2,-10,-7], vec![4,-8,8], vec![3,5,-1]]; //test inputs

    let mut velocity = vec![vec![0; 3]; 4];

    // println!("{:?}\n{:?}",positions, velocity);
    
    let mut x: u128=0;
    let mut y: u128=0;
    let mut z: u128=0;

    let n =0;
    'master: for steps in 0.. {
        let tmp = history.get(&(positions.clone(),velocity.clone()));
        if tmp.is_none() {
            // history.insert((positions,velocity),steps);
            history.insert((positions.clone(),velocity.clone()));
        }
        else{
            let count = tmp.unwrap();
            println!("{}",steps);
            x= steps;
            break 'master;
        }  
        for (moon, mut speed) in positions.iter().zip(velocity.iter_mut()){
            for moon2 in positions.clone() {
                if *moon == moon2 {
                    continue;
                }
                
                if(moon[n]>moon2[n]){
                    speed[n]-=1;
                }
                else if(moon[n]<moon2[n]){
                    speed[n]+=1;
                }
            }
        }

        for (mut moon, speed) in positions.iter_mut().zip(velocity.iter_mut()){
            // for n in 0..3 {
            moon[n]+=speed[n];
            // }
        }
    }

    let n =1;
    history.clear();
    'master: for steps in 0.. {
        let tmp = history.get(&(positions.clone(),velocity.clone()));
        if tmp.is_none() {
            // history.insert((positions,velocity),steps);
            history.insert((positions.clone(),velocity.clone()));
        }
        else{
            let count = tmp.unwrap();
            println!("{}",steps);
            y= steps;
            break 'master;
        }  
        for (moon, mut speed) in positions.iter().zip(velocity.iter_mut()){
            for moon2 in positions.clone() {
                if *moon == moon2 {
                    continue;
                }
                
                if(moon[n]>moon2[n]){
                    speed[n]-=1;
                }
                else if(moon[n]<moon2[n]){
                    speed[n]+=1;
                }
            }
        }

        for (mut moon, speed) in positions.iter_mut().zip(velocity.iter_mut()){
            // for n in 0..3 {
            moon[n]+=speed[n];
            // }
        }
    }

    let n =2;
    history.clear();
    'master: for steps in 0.. {
        let tmp = history.get(&(positions.clone(),velocity.clone()));
        if tmp.is_none() {
            // history.insert((positions,velocity),steps);
            history.insert((positions.clone(),velocity.clone()));
        }
        else{
            let count = tmp.unwrap();
            println!("{}",steps);
            z= steps;
            break 'master;
        }  
        for (moon, mut speed) in positions.iter().zip(velocity.iter_mut()){
            for moon2 in positions.clone() {
                if *moon == moon2 {
                    continue;
                }
                
                if(moon[n]>moon2[n]){
                    speed[n]-=1;
                }
                else if(moon[n]<moon2[n]){
                    speed[n]+=1;
                }
            }
        }

        for (mut moon, speed) in positions.iter_mut().zip(velocity.iter_mut()){
            // for n in 0..3 {
            moon[n]+=speed[n];
            // }
        }
    }


    let time = lcm(x,y);
    let time2 = lcm(time, z);
    println!("{}", time2);
    // println!("{:?}\n{:?}",positions, velocity);

    // let mut energy = 0;
    // for (moon, speed) in positions.iter().zip(velocity.clone()) {
    //     let mut pot = 0;
    //     let mut ken = 0;
    //     for n in 0..3 {
    //         pot += (moon[n] as i32).abs();
    //         ken += (speed[n] as i32).abs();
    //     }


    //     energy+= pot*ken;
    // }

    // println!("{}", energy);
}