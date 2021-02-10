use crate::schema_set::Sql;
use postgres_parser::sys::CmdType;

impl Sql for CmdType {
    fn sql(&self) -> String {
        match self {
            CmdType::CMD_UNKNOWN => "UNKNOWN",
            CmdType::CMD_SELECT => "SELECT",
            CmdType::CMD_UPDATE => "UPDATE",
            CmdType::CMD_INSERT => "INSERT",
            CmdType::CMD_DELETE => "DELETE",
            CmdType::CMD_UTILITY => "UTILITY",
            CmdType::CMD_NOTHING => "NOTHING",
        }
        .into()
    }
}
