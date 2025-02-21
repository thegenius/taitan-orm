input_name: 002
db_type: Postgres
sql_type: Insert
expected: 'let fields = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str ("name,\"select\",\"and\"") ; has_prev = true ; if ! self . age . is_none () { s . push_str (",age" . as_ref ()) ; } if ! self . primary . is_none () { s . push_str (",user_name" . as_ref ()) ; } ; s } ; let marks = { let mut s = String :: default () ; let mut has_prev = false ; let mut index = 1 ; s . push_str ("$1,$2,$3") ; has_prev = true ; index = index + 3usize ; if self . age . is_some () { s . push_str (format ! (",${}" , index) . as_ref ()) ; index = index + 1 ; } if self . primary . is_some () { s . push_str (format ! (",${}" , index) . as_ref ()) ; index = index + 1 ; } ; s } ; format ! ("INSERT INTO \"user\" ({}) VALUES({})" , fields , marks)'




