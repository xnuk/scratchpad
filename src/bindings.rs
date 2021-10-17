use ::gio_sys::{GCancellable, GInputStream};
use ::glib::ffi::{GArray, GError};
use ::std::os::raw::c_char;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Database {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Query {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Record {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SummaryInfo {
	_unused: [u8; 0],
}
impl ResultError {
	pub const SUCCESS: ResultError = ResultError(0);
	pub const ACCESS_DENIED: ResultError = ResultError(1);
	pub const INVALID_HANDLE: ResultError = ResultError(2);
	pub const NOT_ENOUGH_MEMORY: ResultError = ResultError(3);
	pub const INVALID_DATA: ResultError = ResultError(4);
	pub const OUTOFMEMORY: ResultError = ResultError(5);
	pub const INVALID_PARAMETER: ResultError = ResultError(6);
	pub const OPEN_FAILED: ResultError = ResultError(7);
	pub const CALL_NOT_IMPLEMENTED: ResultError = ResultError(8);
	pub const MORE_DATA: ResultError = ResultError(9);
	pub const NOT_FOUND: ResultError = ResultError(10);
	pub const CONTINUE: ResultError = ResultError(11);
	pub const UNKNOWN_PROPERTY: ResultError = ResultError(12);
	pub const BAD_QUERY_SYNTAX: ResultError = ResultError(13);
	pub const INVALID_FIELD: ResultError = ResultError(14);
	pub const FUNCTION_FAILED: ResultError = ResultError(15);
	pub const INVALID_TABLE: ResultError = ResultError(16);
	pub const DATATYPE_MISMATCH: ResultError = ResultError(17);
	pub const INVALID_DATATYPE: ResultError = ResultError(18);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ResultError(pub u32);
impl PropertyType {
	pub const EMPTY: PropertyType = PropertyType(0);
	pub const INT: PropertyType = PropertyType(1);
	pub const STRING: PropertyType = PropertyType(2);
	pub const FILETIME: PropertyType = PropertyType(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PropertyType(pub u32);
impl ColInfo {
	pub const NAMES: ColInfo = ColInfo(0);
	pub const TYPES: ColInfo = ColInfo(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ColInfo(pub u32);
impl DbFlags {
	pub const READONLY: DbFlags = DbFlags(1);
	pub const CREATE: DbFlags = DbFlags(2);
	pub const TRANSACT: DbFlags = DbFlags(4);
	pub const PATCH: DbFlags = DbFlags(8);
}
impl ::core::ops::BitOr<DbFlags> for DbFlags {
	type Output = Self;
	#[inline]
	fn bitor(self, other: Self) -> Self {
		DbFlags(self.0 | other.0)
	}
}
impl ::core::ops::BitOrAssign for DbFlags {
	#[inline]
	fn bitor_assign(&mut self, rhs: DbFlags) {
		self.0 |= rhs.0;
	}
}
impl ::core::ops::BitAnd<DbFlags> for DbFlags {
	type Output = Self;
	#[inline]
	fn bitand(self, other: Self) -> Self {
		DbFlags(self.0 & other.0)
	}
}
impl ::core::ops::BitAndAssign for DbFlags {
	#[inline]
	fn bitand_assign(&mut self, rhs: DbFlags) {
		self.0 &= rhs.0;
	}
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DbFlags(pub u32);
impl DBError {
	pub const SUCCESS: DBError = DBError(0);
	pub const INVALIDARG: DBError = DBError(1);
	pub const MOREDATA: DBError = DBError(2);
	pub const FUNCTIONERROR: DBError = DBError(3);
	pub const DUPLICATEKEY: DBError = DBError(4);
	pub const REQUIRED: DBError = DBError(5);
	pub const BADLINK: DBError = DBError(6);
	pub const OVERFLOW: DBError = DBError(7);
	pub const UNDERFLOW: DBError = DBError(8);
	pub const NOTINSET: DBError = DBError(9);
	pub const BADVERSION: DBError = DBError(10);
	pub const BADCASE: DBError = DBError(11);
	pub const BADGUID: DBError = DBError(12);
	pub const BADWILDCARD: DBError = DBError(13);
	pub const BADIDENTIFIER: DBError = DBError(14);
	pub const BADLANGUAGE: DBError = DBError(15);
	pub const BADFILENAME: DBError = DBError(16);
	pub const BADPATH: DBError = DBError(17);
	pub const BADCONDITION: DBError = DBError(18);
	pub const BADFORMATTED: DBError = DBError(19);
	pub const BADTEMPLATE: DBError = DBError(20);
	pub const BADDEFAULTDIR: DBError = DBError(21);
	pub const BADREGPATH: DBError = DBError(22);
	pub const BADCUSTOMSOURCE: DBError = DBError(23);
	pub const BADPROPERTY: DBError = DBError(24);
	pub const MISSINGDATA: DBError = DBError(25);
	pub const BADCATEGORY: DBError = DBError(26);
	pub const BADKEYTABLE: DBError = DBError(27);
	pub const BADMAXMINVALUES: DBError = DBError(28);
	pub const BADCABINET: DBError = DBError(29);
	pub const BADSHORTCUT: DBError = DBError(30);
	pub const STRINGOVERFLOW: DBError = DBError(31);
	pub const BADLOCALIZEATTRIB: DBError = DBError(32);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DBError(pub u32);
impl Property {
	pub const DICTIONARY: Property = Property(0);
	pub const CODEPAGE: Property = Property(1);
	pub const TITLE: Property = Property(2);
	pub const SUBJECT: Property = Property(3);
	pub const AUTHOR: Property = Property(4);
	pub const KEYWORDS: Property = Property(5);
	pub const COMMENTS: Property = Property(6);
	pub const TEMPLATE: Property = Property(7);
	pub const LASTAUTHOR: Property = Property(8);
	pub const UUID: Property = Property(9);
	pub const EDITTIME: Property = Property(10);
	pub const LASTPRINTED: Property = Property(11);
	pub const CREATED_TM: Property = Property(12);
	pub const LASTSAVED_TM: Property = Property(13);
	pub const VERSION: Property = Property(14);
	pub const SOURCE: Property = Property(15);
	pub const RESTRICT: Property = Property(16);
	pub const THUMBNAIL: Property = Property(17);
	pub const APPNAME: Property = Property(18);
	pub const SECURITY: Property = Property(19);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Property(pub u32);
#[link(name = "msi")]
extern "C" {
	#[link_name = "libmsi_database_new"]
	pub fn database_new(
		path: *const c_char,
		flags: u32,
		persist: *const ::std::os::raw::c_char,
		error: *mut *mut GError,
	) -> *mut Database;
	#[link_name = "libmsi_database_is_readonly"]
	pub fn database_is_readonly(db: *mut Database) -> bool;
	#[link_name = "libmsi_database_get_primary_keys"]
	pub fn database_get_primary_keys(
		db: *mut Database,
		table: *const ::std::os::raw::c_char,
		error: *mut *mut GError,
	) -> *mut Record;
	#[link_name = "libmsi_database_apply_transform"]
	pub fn database_apply_transform(
		db: *mut Database,
		file: *const ::std::os::raw::c_char,
		error: *mut *mut GError,
	) -> bool;
	#[link_name = "libmsi_database_export"]
	pub fn database_export(
		db: *mut Database,
		table: *const ::std::os::raw::c_char,
		fd: ::std::os::raw::c_int,
		error: *mut *mut GError,
	) -> bool;
	#[link_name = "libmsi_database_import"]
	pub fn database_import(
		db: *mut Database,
		path: *const ::std::os::raw::c_char,
		error: *mut *mut GError,
	) -> bool;
	#[link_name = "libmsi_database_is_table_persistent"]
	pub fn database_is_table_persistent(
		db: *mut Database,
		table: *const ::std::os::raw::c_char,
		error: *mut *mut GError,
	) -> bool;
	#[link_name = "libmsi_database_merge"]
	pub fn database_merge(
		db: *mut Database,
		merge: *mut Database,
		table: *const ::std::os::raw::c_char,
		error: *mut *mut GError,
	) -> bool;
	#[link_name = "libmsi_database_commit"]
	pub fn database_commit(db: *mut Database, error: *mut *mut GError) -> bool;
	#[link_name = "libmsi_query_new"]
	pub fn query_new(
		database: *mut Database,
		query: *const c_char,
		error: *mut *mut GError,
	) -> *mut Query;
	#[link_name = "libmsi_query_fetch"]
	pub fn query_fetch(
		query: *mut Query,
		error: *mut *mut GError,
	) -> *mut Record;
	#[link_name = "libmsi_query_execute"]
	pub fn query_execute(
		query: *mut Query,
		rec: *mut Record,
		error: *mut *mut GError,
	) -> bool;
	#[link_name = "libmsi_query_close"]
	pub fn query_close(query: *mut Query, error: *mut *mut GError) -> bool;
	#[link_name = "libmsi_query_get_column_info"]
	pub fn query_get_column_info(
		query: *mut Query,
		info: ColInfo,
		error: *mut *mut GError,
	) -> *mut Record;
	#[link_name = "libmsi_record_new"]
	pub fn record_new(count: u32) -> *mut Record;
	#[link_name = "libmsi_record_clear"]
	pub fn record_clear(record: *mut Record) -> bool;
	#[link_name = "libmsi_record_get_field_count"]
	pub fn record_get_field_count(record: *const Record) -> u32;
	#[link_name = "libmsi_record_is_null"]
	pub fn record_is_null(record: *const Record, field: u32) -> bool;
	#[link_name = "libmsi_record_set_int"]
	pub fn record_set_int(record: *mut Record, field: u32, val: i64) -> bool;
	#[link_name = "libmsi_record_get_int"]
	pub fn record_get_int(record: *const Record, field: u32) -> i64;
	#[link_name = "libmsi_record_set_string"]
	pub fn record_set_string(
		record: *mut Record,
		field: u32,
		val: *const c_char,
	) -> bool;
	#[link_name = "libmsi_record_get_string"]
	pub fn record_get_string(record: *const Record, field: u32) -> *mut c_char;
	#[link_name = "libmsi_record_load_stream"]
	pub fn record_load_stream(
		record: *mut Record,
		field: u32,
		filename: *const c_char,
	) -> bool;
	#[link_name = "libmsi_record_set_stream"]
	pub fn record_set_stream(
		record: *mut Record,
		field: u32,
		input: *mut GInputStream,
		count: u64,
		cancellable: *mut GCancellable,
		error: *mut *mut GError,
	) -> bool;
	#[link_name = "libmsi_record_get_stream"]
	pub fn record_get_stream(
		record: *mut Record,
		field: u32,
	) -> *mut GInputStream;
	#[link_name = "libmsi_summary_info_new"]
	pub fn summary_info_new(
		database: *mut Database,
		update_count: ::std::os::raw::c_uint,
		error: *mut *mut GError,
	) -> *mut SummaryInfo;
	#[link_name = "libmsi_summary_info_get_property_type"]
	pub fn summary_info_get_property_type(
		si: *mut SummaryInfo,
		prop: Property,
		error: *mut *mut GError,
	) -> PropertyType;
	#[link_name = "libmsi_summary_info_get_string"]
	pub fn summary_info_get_string(
		si: *mut SummaryInfo,
		prop: Property,
		error: *mut *mut GError,
	) -> *const c_char;
	#[link_name = "libmsi_summary_info_get_int"]
	pub fn summary_info_get_int(
		si: *mut SummaryInfo,
		prop: Property,
		error: *mut *mut GError,
	) -> i64;
	#[link_name = "libmsi_summary_info_get_filetime"]
	pub fn summary_info_get_filetime(
		si: *mut SummaryInfo,
		prop: Property,
		error: *mut *mut GError,
	) -> u64;
	#[link_name = "libmsi_summary_info_set_string"]
	pub fn summary_info_set_string(
		si: *mut SummaryInfo,
		prop: Property,
		value: *const c_char,
		error: *mut *mut GError,
	) -> bool;
	#[link_name = "libmsi_summary_info_set_int"]
	pub fn summary_info_set_int(
		si: *mut SummaryInfo,
		prop: Property,
		value: i64,
		error: *mut *mut GError,
	) -> bool;
	#[link_name = "libmsi_summary_info_set_filetime"]
	pub fn summary_info_set_filetime(
		si: *mut SummaryInfo,
		prop: Property,
		value: u64,
		error: *mut *mut GError,
	) -> bool;
	#[link_name = "libmsi_summary_info_persist"]
	pub fn summary_info_persist(
		si: *mut SummaryInfo,
		error: *mut *mut GError,
	) -> bool;
	#[link_name = "libmsi_summary_info_save"]
	pub fn summary_info_save(
		si: *mut SummaryInfo,
		db: *mut Database,
		error: *mut *mut GError,
	) -> bool;
	#[link_name = "libmsi_summary_info_get_properties"]
	pub fn summary_info_get_properties(si: *mut SummaryInfo) -> *mut GArray;
}
