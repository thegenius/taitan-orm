input_name: 001
db_type: Postgres
sql_type: Insert
expected: >-
  let fields = {
  let mut s = String :: default () ;
  let mut has_prev = false ;
  s . push_str ("a,b,c") ;
  has_prev = true ;
  if ! self . d . is_none () { s . push_str (",d") ; }
  if ! self . e . is_none () { s . push_str (",user_name") ;
  } s } ;
  let marks = { let mut s = String :: default () ;
  let mut has_prev = false ;
  let mut index = 0 ;
  s . push_str ("$1,$2,$3") ; has_prev = true ; index += 3usize ;
  if ! self . d . is_none () { { index += 1 ; s . push_str (format ! (",${}" , index) . as_ref ()) } }
  if ! self . e . is_none () { { index += 1 ; s . push_str (format ! (",${}" , index) . as_ref ()) } }
  s } ;
  std :: borrow :: Cow :: Owned (format ! ("INSERT INTO \"user\" ({}) VALUES({})" , fields , marks))