#![allow(clippy::indexing_slicing)]
// Using the `#!` to suppress the warning of `clippy::indexing_slicing` in the generated code.
use bitfield::bitfield;

bitfield! {
    pub struct CmdQueueDescCommonHead([u8]);
    u32;
    pub get_valid , set_valid: 0;
    pub get_is_success_or_need_signal_cplt, set_is_success_or_need_signal_cplt: 1;
    pub get_op_code, set_op_code: 7, 2;
    pub get_extra_segment_cnt, set_extra_segment_cnt: 11, 8;
    pub _reserverd, set_reserverd: 31, 12;
    pub get_user_data, set_user_data: 63, 32;
}

bitfield! {
    pub struct CmdQueueReqDescUpdateMrTable([u8]);
    u64;
    _cmd_queue_desc_common_head,_: 63, 0;      // 64bits
    pub get_mr_base_va, set_mr_base_va: 127, 64;   // 64bits
    pub get_mr_length, set_mr_length: 159, 128;    // 32bits
    pub get_mr_key, set_mr_key: 191, 160;          // 32bits
    pub get_pd_handler, set_pd_handler: 223, 192;  // 32bits
    pub get_acc_flags, set_acc_flags: 231, 224;    // 8bits
    pub get_pgt_offset, set_pgt_offset: 248, 232;  // 17bits
    _reserved0, _: 255, 249;                   // 7bits
}

bitfield! {
    pub struct CmdQueueReqDescUpdatePGT([u8]);
    u64;
    __cmd_queue_desc_common_head,_ : 63, 0;             // 64bits
    pub get_dma_addr, set_dma_addr: 127, 64;                // 64bits
    pub get_start_index, set_start_index: 159, 128;         // 32bits
    pub get_dma_read_length, set_dma_read_length: 191, 160; // 32bits
    _reserved0, _: 255, 192;                            // 64bits
}

bitfield! {
    pub struct CmdQueueReqDescQpManagementSeg0([u8]);
    u64;
    _cmd_queue_desc_common_head,_: 63, 0;                                       // 64bits
    pub get_is_valid, set_is_valid: 64;                                             // 1bit
    pub get_is_error, set_is_error: 65;                                             // 1bit
    _reserverd4, _: 71, 66;                                                     // 6bits
    pub get_qpn, set_qpn: 95, 72;                                                   // 24bits
    pub get_pd_handler, set_pd_handler: 127, 96;                                    // 32bits
    pub get_qp_type, set_qp_type: 131, 128;                                         // 4bits
    _reserverd3, _: 135, 132;                                                   // 4bits
    pub get_rq_access_flags, set_rq_access_flags: 143, 136;                         // 8bits
    pub get_pmtu, set_pmtu: 146, 144;                                               // 3bits
    _reserverd2, _: 151, 147;                                                   // 5bits
    _reserverd1, _: 255, 152;                                                   // 104bits
}

bitfield! {
    pub struct CmdQueueReqDescSetNetworkParam([u8]);
    u64;
    _cmd_queue_desc_common_head,_:          63 ,   0;                                       // 64bits
    pub get_ip_gateway, set_ip_gateway:         95 ,  64;                                       // 32bits
    pub get_ip_netmask, set_ip_netmask:         127,  96;                                       // 32bit
    pub get_ip_addr, set_ip_addr:               159, 128;                                       // 32bit
    _reserverd1, _:                         191, 160;                                       // 32bit
    pub get_eth_mac_addr, set_eth_mac_addr:     239, 192;                                       // 48bit
    _reserverd2, _:                         255, 240;                                       // 16bit
}

bitfield! {
    pub struct CmdQueueReqDescSetRawPacketReceiveMeta([u8]);
    u64;
    _cmd_queue_desc_common_head,_:              63 ,   0;                                   // 64bits
    pub get_write_base_addr, set_write_base_addr:   127,  64;                                   // 64bits
    pub get_write_mr_key, set_write_mr_key:         159, 128;                                   // 32bits
    _reserverd1, _:                             191, 160;                                   // 32bits
    _reserverd2, _:                             255, 240;                                   // 64bits
}

bitfield! {
    pub struct SendQueueDescCommonHead([u8]);
    u32;
    pub get_valid , set_valid: 0;                                                  // 1bit
    pub get_is_success_or_need_signal_cplt, set_is_success_or_need_signal_cplt: 1; // 1bit
    pub get_is_first, set_is_first: 2;                                             // 1bit
    pub get_is_last, set_is_last: 3;                                               // 1bit
    pub get_op_code, set_op_code: 7, 4;                                            // 4bits
    pub get_extra_segment_cnt, set_extra_segment_cnt: 11, 8;                       // 4bits
    _reserverd, _: 31, 12;                                                     // 20bits
    pub get_total_len, set_total_len: 63, 32;                                      // 32bits
}

bitfield! {
    pub struct SendQueueReqDescSeg0([u8]);
    u64;
    _common_header, _: 63, 0;         // 64bits
    pub get_raddr, set_raddr: 127, 64;    // 64bits
    pub get_rkey, set_rkey: 159, 128;     // 32bits
    pub get_dqp_ip, set_dqp_ip: 191, 160; // 32bits
    pub get_pkey, set_pkey: 207, 192;     // 16bits
    _reserverd, _: 255, 208;          // 48bits
}

bitfield! {
    pub struct SendQueueReqDescSeg1([u8]);
    u64;
    pub get_pmtu, set_pmtu: 2, 0;             // 3bits
    _reserved8 , _: 7, 3;                 // 5bits
    pub get_flags, set_flags: 12, 8;          // 5bits
    _reserved7 , _: 15, 13;               // 3bits
    pub get_qp_type, set_qp_type: 19, 16;     // 4bits
    _reserved6 , _: 23, 20;               // 4bits
    pub get_seg_cnt, set_seg_cnt: 26, 24;     // 3bits
    _reserved5 , _: 31, 27;               // 5bits
    pub get_psn, set_psn: 55, 32;             // 24bits
    _reserved4 , _: 63, 56;               // 8bits
    pub get_mac_addr, set_mac_addr: 111, 64;  // 48bits
    _reserved3 , _: 127, 112;             // 16bits
    pub get_dqpn, set_dqpn: 151, 128;         // 24bits
    _reserved2 , _: 159, 152;             // 8bits
    pub get_imm, set_imm: 191, 160;           // 32bits
    _reserved1 , _: 255, 192;             // 64bits
}

bitfield! {
    pub struct SendQueueReqDescFragSGE([u8]);
    u64;
    pub get_lkey, set_lkey: 31, 0;     // 32bits
    pub get_len, set_len: 63, 32;      // 32bits
    pub get_laddr, set_laddr: 127, 64; // 64bits
}

bitfield! {
    pub struct MeatReportQueueDescFragRETH([u8]);
    u64;
    pub get_va, set_va: 63, 0;          // 64bits
    pub get_rkey, set_rkey: 95, 64;     // 32bits
    pub get_dlen, set_dlen: 127, 96;    // 32bits
}

bitfield! {
    pub struct MeatReportQueueDescFragImmDT([u8]);
    u32;
    pub get_imm, set_imm: 32, 0;          // 32bits
}

bitfield! {
    pub struct MeatReportQueueDescFragAETH([u8]);
    u32;
    pub get_psn, set_psn: 23, 0;          // 24bits
    pub get_msn, set_msn: 47, 24;         // 24bits
    pub get_aeth_value, set_aeth_value: 52, 48; // 5bits
    pub get_aeth_code, set_aeth_code: 55, 53;   // 3bits
}

bitfield! {
    pub struct MeatReportQueueDescBthReth([u8]);
    u64;
    pub get_expected_psn, _: 23,0;      // 24bits
    pub get_req_status, _: 31,24;       // 8bit
    pub get_bth, _: 95, 32;             // 64bits
    pub get_reth, _: 223, 96;           // 128bits
    pub get_msn, _: 247,224;            // 24bits
    reserved1,_ : 255, 248;         // 8bits
}

bitfield! {
    pub struct MeatReportQueueDescFragBTH([u8]);
    u32;
    pub get_trans_type,set_trans_type: 2, 0; // 3bits
    pub get_opcode,set_opcode: 7, 3;         // 5bits
    pub get_qpn,set_qpn: 31, 8;              // 24bits
    pub get_psn,set_psn: 55, 32;             // 24bits
    pub get_solicited,set_solicited: 56;     // 1bit
    pub get_ack_req,set_ack_req: 57;         // 1bit
    pub get_pad_cnt,set_pad_cnt: 63, 58;     // 4bits
}

bitfield! {
    pub struct MeatReportQueueDescFragSecondaryRETH([u8]);
    u64;
    pub get_secondary_va,set_secondary_va: 63, 0; // 64bits
    pub get_secondary_rkey,set_secondary_rkey: 95, 64; // 32bits
}

bitfield! {
    /// IPv4 layout
    pub struct Ipv4([u8]);
    u32;
    pub get_version_and_len,set_version_and_len: 7, 0;         // 8bits
    pub get_dscp_ecn,set_dscp_ecn: 15, 8;                      // 8bits
    pub get_total_length,set_total_length: 31, 16;             // 16bits
    pub get_identification,set_identification: 47, 32;         // 16bits
    pub get_fragment_offset,set_fragment_offset: 63, 48;       // 16bits
    pub get_ttl,set_ttl: 71, 64;                               // 8bits
    pub get_protocol,set_protocol: 79, 72;                     // 8bits
    pub get_checksum,set_checksum: 95, 80;                     // 16bits
    pub get_source,set_source: 127, 96;                        // 32bits
    pub get_destination,set_destination: 159, 128;             // 32bits
}

bitfield! {
    /// UDP layout
    pub struct Udp([u8]);
    u16;
    pub get_src_port,set_src_port: 15, 0;                      // 16bits
    pub get_dst_port,set_dst_port: 31, 16;                     // 16bits
    pub get_length,set_length: 47, 32;                         // 16bits
    pub get_checksum,set_checksum: 63, 48;                     // 16bits
}

bitfield! {
    /// BTH layout
    pub struct Bth([u8]);
    u32;
    pub get_opcode,set_opcode: 7, 0;         // 8bits
    _padding_0,_ : 9, 8;                 // 2bits
    pub get_pad_count,set_pad_count: 11, 10; // 2bits
    _padding_1,_ : 15, 12;               // 4bits
    pub get_pkey,set_pkey: 31, 16;           // 16bits
    pub _,set_ecn_and_resv6: 39, 32;         // 8bits
    pub get_dqpn,set_dqpn: 63, 40;           // 24bits
    _padding_2,_ : 71, 64;               // 8bits
    pub get_psn,set_psn: 95, 72;             // 24bits
}

bitfield! {
    /// Aeth layout
    pub struct Aeth([u8]);
    u32;
    _padding_0,_ : 0;                     // 1bits
    pub get_aeth_code,set_aeth_code: 2, 1;    // 2bits
    pub get_aeth_value,set_aeth_value: 7, 3;  // 5bits
    _padding_1,_ :   15,8;               // 8bits
    pub get_msn,set_msn: 31,16;               // 16bits
}

bitfield! {
    /// Nak Retry Eth layout
    pub struct NReth([u8]);
    u32;
    pub get_last_retry_psn,set_last_retry_psn: 23, 0; // 24bits
    _padding_0,_: 31, 24;                         // 8its
}