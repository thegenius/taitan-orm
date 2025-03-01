input_name: 002
db_type: Postgres
sql_type: Insert
expected: >-
  let fields = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str ("name,\"select\",\"and\"") ; has_prev = true ; has_prev = true ; if ! self . age . is_none () { s . push_str (",age") ; } if ! self . primary . is_none () { s . push_str (",user_name") ; } s } ; let marks = { let mut s = String :: default () ; let mut has_prev = false ; let mut index = 0 ; s . push_str ("$1,$2,$3") ; has_prev = true ; has_prev = true ; index += 3usize ; if ! self . age . is_none () { { index += 1 ; s . push_str (format ! (",${}" , index) . as_ref ()) } } if ! self . primary . is_none () { { index += 1 ; s . push_str (format ! (",${}" , index) . as_ref ()) } } s } ; std :: borrow :: Cow :: Owned (format ! ("INSERT INTO \"user\" ({}) VALUES({})" , fields , marks))





