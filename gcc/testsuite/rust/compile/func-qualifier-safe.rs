// { dg-additional-options "-frust-edition=2021" }
#![feature(no_core)]
#![no_core]

unsafe extern "C" {
    safe fn trust_me_bro();
}

unsafe fn disallowed() -> i64 { 42 }
