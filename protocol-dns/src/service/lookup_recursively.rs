use crate::dns::{DnsPacket, QueryType, Result, ResultCode};
use crate::service::lookup;
use std::net::Ipv4Addr;

pub fn lookup_recursively(
    qname: &str,
    qtype: QueryType,
    ns_default: Ipv4Addr,
) -> Result<DnsPacket> {
    let mut ns = ns_default.clone();
    loop {
        println!("attempting lookup of {qtype:?} {qname} with ns {ns}");
        let ns_copy = ns;
        let server = (ns_copy, 53);
        let response = lookup(qname, qtype, server)?;

        if !response.answers.is_empty() && response.header.rescode == ResultCode::NOERROR {
            return Ok(response);
        }

        if response.header.rescode == ResultCode::NXDOMAIN {
            return Ok(response);
        }

        if let Some(new_ns) = response.get_resolve_ns(qname) {
            ns = new_ns;
            continue;
        }

        let Some(new_ns_name) = response.get_unresolved_ns(qname) else {
            return Ok(response);
        };

        let recursive_response = lookup_recursively(&new_ns_name, QueryType::A, ns_default)?;
        if let Some(new_ns) = recursive_response.get_random_a() {
            ns = new_ns;
        } else {
            return Ok(response);
        }
    }
}
