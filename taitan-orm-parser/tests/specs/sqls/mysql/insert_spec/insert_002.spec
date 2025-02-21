input_name: 002
db_type: MySql
sql_type: Insert
expected: 'let fields = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str ("name,`select`,`and`") ; has_prev = true ; if ! self . age . is_none () { s . push_str (",age" . as_ref ()) ; } if ! self . primary . is_none () { s . push_str (",user_name" . as_ref ()) ; } ; s } ; let marks = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str ("?,?,?") ; has_prev = true ; if ! self . age . is_none () { s . push_str (",?" . as_ref ()) ; } if ! self . primary . is_none () { s . push_str (",?" . as_ref ()) ; } ; s } ; format ! ("INSERT INTO user ({}) VALUES({})" , fields , marks)'




