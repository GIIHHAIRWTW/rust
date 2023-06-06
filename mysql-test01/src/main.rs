use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct Reader {
    readerno: String,
    readertype: String,
    readername: String,
    readerdept: String,
    borrowed_num: u32,
}

fn main() {
    let url = "mysql://root:root@localhost:3306/books";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let res = conn
        .query_map(
            "select * from reader",
            |(readerno, readertype, readername, readerdept, borrowed_num)| Reader {
                readerno,
                readertype,
                readername,
                readerdept,
                borrowed_num,
            },
        )
        .expect("Query failed.");

    for i in res {
        println!("{:?}", i);
    }
}
