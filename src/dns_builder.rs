use dns_message_parser::question::{QClass, QType, Question};
use dns_message_parser::{Dns, DomainName, Flags, Opcode, RCode};
use std::convert::TryFrom;

pub struct DnsBuilder;

impl DnsBuilder {
    pub fn get_query_packet(domain: &str) -> Vec<u8> {
        let id = 56092;
        let flags = Flags {
            qr: true,
            opcode: Opcode::Query,
            aa: true,
            tc: false,
            rd: true,
            ra: true,
            ad: false,
            cd: false,
            rcode: RCode::NoError,
        };

        let q_a = {
            let domain_name = DomainName::try_from(domain).unwrap();
            let q_class = QClass::IN;
            let q_type = QType::A;

            Question {
                domain_name,
                q_class,
                q_type,
            }
        };

        let questions = vec![q_a];

        let dns = Dns {
            id,
            flags,
            questions,
            answers: Vec::new(),
            authorities: Vec::new(),
            additionals: Vec::new(),
        };

        let bytes = dns.encode().unwrap();
        let as_arr: Vec<u8> = bytes.to_vec();

        as_arr
    }
}
