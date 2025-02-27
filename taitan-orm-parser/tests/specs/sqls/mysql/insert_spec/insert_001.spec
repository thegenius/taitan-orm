input_name: 001
db_type: MySql
sql_type: Insert
expected: >-
  let fields = { let mut s = String :: default () ;
  let mut has_prev = false ;
  s . push_str ("a,b,c") ;
  has_prev = true ;
  if ! self . d . is_none () {
  s . push_str (",d") ; }
  if ! self . e . is_none () {
  s . push_str (",user_name") ; }
  s } ;
  let marks = { let mut s = String :: default () ;
  let mut has_prev = false ;
  s . push_str ("?,?,?") ;
  has_prev = true ;
  if ! self . d . is_none () {
  s . push_str (",?") ; }
  if ! self . e . is_none () {
  s . push_str (",?") ; } s } ;
  std :: borrow :: Cow :: Owned (format ! ("INSERT INTO user ({}) VALUES({})" , fields , marks))