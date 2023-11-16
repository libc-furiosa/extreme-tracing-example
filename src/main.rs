#[chrometracer::instrument]
fn bar() {
    println!("hello");
}

#[chrometracer::instrument]
fn foo() {
    bar();
    bar();
}

fn main() {
    let _guard = chrometracer::builder().init();
    foo();
}

/*
 * result sample

[
{"name":"bar","ts":103.101,"dur":10.502,"pid":602505,"tid":1,"ph":"X"},
{"name":"bar","ts":122.241,"dur":2.866,"pid":602505,"tid":1,"ph":"X"},
{"name":"foo","ts":102.968,"dur":22.339,"pid":602505,"tid":1,"ph":"X"}
]
 
 */
