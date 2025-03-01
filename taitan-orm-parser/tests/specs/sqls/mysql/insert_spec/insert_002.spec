input_name: 002
db_type: MySql
sql_type: Insert
expected: >-
  let fields = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str ("name,`select`,`and`") ; has_prev = true ; has_prev = true ; if ! self . age . is_none () { s . push_str (",age") ; } if ! self . primary . is_none () { s . push_str (",user_name") ; } s } ; let marks = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str ("?,?,?") ; has_prev = true ; has_prev = true ; if ! self . age . is_none () { s . push_str (",?") ; } if ! self . primary . is_none () { s . push_str (",?") ; } s } ; std :: borrow :: Cow :: Owned (format ! ("INSERT INTO user ({}) VALUES({})" , fields , marks))

