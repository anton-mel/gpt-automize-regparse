#[derive(FromBytes)]
#[repr(C)]
pub struct Registers {
    /// Device Control Register
    device_control_register: Volatile<u32>, // 0x0
    _padding0: [u8; 4], // 0x4 - 0x7

    /// Device Status Register
    device_status_register: ReadOnly<u32>, // 0x8
    _padding1: [u8; 12], // 0xC - 0x17

    /// Extended Device Control Register
    extended_device_control_register: Volatile<u32>, // 0x18
    _padding2: [u8; 4], // 0x1C - 0x1F

    /// Extended SDP Control
    extended_sdp_control: Volatile<u32>, // 0x20
    _padding3: [u8; 4], // 0x24 - 0x27

    /// I2C Control
    i2c_control: Volatile<u32>, // 0x28
    _padding4: [u8; 32], // 0x2C - 0x4B

    /// TCP Timer
    tcp_timer: Volatile<u32>, // 0x4C
    _padding5: [u8; 432], // 0x50 - 0x1FF

    /// LED Control
    led_control: Volatile<u32>, // 0x200
    _padding6: [u8; 1020], // 0x204 - 0x5FF

    /// PF VFLR Events Indication Target
    pf_vflr_events_indication_target: Reserved<u32>, // 0x600
    pf_vflr_events_clear_target: [Volatile<u32>; 1],
    pf_mailbox_interrupt_causes_registertarget: [Volatile<u32>; 4],
    pf_mailbox_interrupt_mask_registertarget: [Volatile<u32>; 1],
    _padding10: [u8; 508], // 0x604 - 0x7FF

    /// Extended Interrupt Cause Register
    extended_interrupt_cause_register: Reserved<u32>, // 0x800
    _padding11: [u8; 4], // 0x804 - 0x807

    /// Extended Interrupt Cause Set Register
    extended_interrupt_cause_set_register: WriteOnly<u32>, // 0x808
    extended_interrupt_throttle: [Volatile<u32>; 24],
    _padding13: [u8; 116], // 0x80C - 0x87F

    /// Extended Interrupt Mask Set/Read Register
    extended_interrupt_mask_set_read_register: Reserved<u32>, // 0x880
    _padding14: [u8; 4], // 0x884 - 0x887

    /// Extended Interrupt Mask Clear Register
    extended_interrupt_mask_clear_register: WriteOnly<u32>, // 0x888
    _padding15: [u8; 4], // 0x88C - 0x88F

    /// Extended Interrupt Auto Mask Enable
    extended_interrupt_auto_mask_enable: Volatile<u32>, // 0x890
    /// MSI to EITR Select
    msi_to_eitr_select: Volatile<u32>, // 0x894
    /// General Purpose Interrupt Enable
    general_purpose_interrupt_enable: Volatile<u32>, // 0x898
    interrupt_vector_allocation: [Volatile<u32>; 64],
    _padding19: [u8; 356], // 0x89C - 0x9FF

    /// Miscellaneous Interrupt Vector Allocation
    miscellaneous_interrupt_vector_allocation: Volatile<u32>, // 0xA00
    extended_interrupt_cause_set: [Volatile<u32>; 2],
    extended_interrupt_mask_set_read: [WriteOnly<u32>; 2],
    extended_interrupt_mask_clear: [WriteOnly<u32>; 2],
    extended_interrupt_auto_mask_enable: [Volatile<u32>; 2],
    receive_descriptor_base_address_low: [Volatile<u32>; 64],
    receive_descriptor_base_address_high: [Volatile<u32>; 64],
    receive_descriptor_length: [Volatile<u32>; 64],
    rx_dca_control_register: [Volatile<u32>; 64],
    receive_descriptor_head: [ReadOnly<u32>; 64],
    split_receive_control_registers: [Volatile<u32>; 64],
    receive_descriptor_tail: [Volatile<u32>; 64],
    receive_descriptor_control: [Volatile<u32>; 64],
    rsc_control: [Volatile<u32>; 64],
    queue_packets_received_count: [ReadOnly<u32>; 16],
    queue_bytes_received_count_low: [RC<u32>; 16],
    queue_bytes_received_count_high: [RC<u32>; 16],
    queue_packets_received_drop_count: [RC<u32>; 16],
    dcb_receive_packet_plane_t4_config: [Volatile<u32>; 8],
    dcb_receive_packet_plane_t4_status: [ReadOnly<u32>; 8],
    receive_queue_statistic_mapping_registers: [Volatile<u32>; 32],
    _padding40: [u8; 6668], // 0xA04 - 0x240F

    /// FC User Descriptor PTR Low
    fc_user_descriptor_ptr_low: Volatile<u32>, // 0x2410
    /// FC User Descriptor PTR High
    fc_user_descriptor_ptr_high: Volatile<u32>, // 0x2414
    /// FC Buffer Control
    fc_buffer_control: Volatile<u32>, // 0x2418
    /// FC CRC Error Count
    fc_crc_error_count: Reserved<u32>, // 0x241C
    /// FCoE Rx Packets Dropped Count
    fcoe_rx_packets_dropped_count: Reserved<u32>, // 0x241C
    /// FC Receive DMA RW
    fc_receive_dma_rw: Volatile<u32>, // 0x2420
    /// FC Last Error Count
    fc_last_error_count: Reserved<u32>, // 0x2424
    /// FCoE Packets Received Count
    fcoe_packets_received_count: Reserved<u32>, // 0x2428
    /// FCOE DWord Received Count
    fcoe_dword_received_count: Reserved<u32>, // 0x242C
    _padding49: [u8; 2768], // 0x2430 - 0x2EFF

    /// Receive DMA Control Register
    receive_dma_control_register: Volatile<u32>, // 0x2F00
    /// PF Queue Drop Enable Register
    pf_queue_drop_enable_register: Volatile<u32>, // 0x2F04
    _padding51: [u8; 56], // 0x2F08 - 0x2F3F

    /// Rx DMA Statistic Counter Control
    rx_dma_statistic_counter_control: Volatile<u32>, // 0x2F40
    _padding52: [u8; 36], // 0x2F44 - 0x2F67

    /// DMA Good Rx LPBK Packet
    dma_good_rx_lpbk_packet: Reserved<u32>, // 0x2F68
    /// DMA Good Rx LPBK Byte Counter
    dma_good_rx_lpbk_byte_counter: Reserved<u32>, // 0x2F6C
    /// DMA Good Rx LPBK Byte Counter
    dma_good_rx_lpbk_byte_counter: Reserved<u32>, // 0x2F70
    /// DMA Duplicated Good Rx LPBK Packet Counter
    dma_duplicated_good_rx_lpbk_packet_counter: Reserved<u32>, // 0x2F74
    /// DMA Duplicated Good Rx LPBK Byte Counter
    dma_duplicated_good_rx_lpbk_byte_counter: Reserved<u32>, // 0x2F78
    /// DMA Duplicated Good Rx LPBK Byte Counter
    dma_duplicated_good_rx_lpbk_byte_counter: Reserved<u32>, // 0x2F7C
    _padding58: [u8; 128], // 0x2F80 - 0x2FFF

    /// Receive Control Register
    receive_control_register: Volatile<u32>, // 0x3000
    _padding59: [u8; 28], // 0x3004 - 0x301F

    /// DCB Receive User Priority to Traffic Class
    dcb_receive_user_priority_to_traffic_class: Volatile<u32>, // 0x3020
    _padding60: [u8; 4], // 0x3024 - 0x3027

    /// RSC Data Buffer Control Register
    rsc_data_buffer_control_register: Volatile<u32>, // 0x3028
    _padding61: [u8; 356], // 0x302C - 0x318F

    /// Rx Packet Buffer Flush Detect
    rx_packet_buffer_flush_detect: ReadOnly<u32>, // 0x3190
    flow_control_transmit_timer: [Volatile<u32>; 4],
    flow_control_receive_threshold_low: [Volatile<u32>; 8],
    flow_control_receive_threshold_high: [Volatile<u32>; 8],
    _padding65: [u8; 268], // 0x3194 - 0x329F

    /// Flow Control Refresh Threshold Value
    flow_control_refresh_threshold_value: Volatile<u32>, // 0x32A0
    receive_packet_buffer_size: [Volatile<u32>; 8],
    _padding67: [u8; 2652], // 0x32A4 - 0x3CFF

    /// Flow Control Configuration
    flow_control_configuration: Volatile<u32>, // 0x3D00
    _padding68: [u8; 768], // 0x3D04 - 0x4003

    /// Illegal Byte Error Count
    illegal_byte_error_count: Reserved<u32>, // 0x4004
    /// Error Byte Count
    error_byte_count: Reserved<u32>, // 0x4008
    _padding70: [u8; 4], // 0x400C - 0x400F

    /// MAC short Packet Discard Count
    mac_short_packet_discard_count: Reserved<u32>, // 0x4010
    _padding71: [u8; 32], // 0x4014 - 0x4033

    /// MAC Local Fault Count
    mac_local_fault_count: Reserved<u32>, // 0x4034
    /// MAC Remote Fault Count
    mac_remote_fault_count: Reserved<u32>, // 0x4038
    priority_xon_received_count: [ReadOnly<u32>; 4],
    _padding74: [u8; 32], // 0x403C - 0x405B

    /// Packets Received Count 1
    packets_received_count_1: Reserved<u32>, // 0x405C
    /// Packets Received Count 2
    packets_received_count_2: Reserved<u32>, // 0x4060
    /// Packets Received Count 3
    packets_received_count_3: Reserved<u32>, // 0x4064
    /// Packets Received Count 4
    packets_received_count_4: Reserved<u32>, // 0x4068
    _padding78: [u8; 20], // 0x406C - 0x407F

    /// Good Packets Transmitted Count
    good_packets_transmitted_count: ReadOnly<u32>, // 0x4080
    _padding79: [u8; 12], // 0x4084 - 0x408F

    /// Good Octets Transmitted Count Low
    good_octets_transmitted_count_low: Reserved<u32>, // 0x4090
    /// Good Octets Transmitted Count High
    good_octets_transmitted_count_high: Reserved<u32>, // 0x4094
    _padding81: [u8; 16], // 0x4098 - 0x40A7

    /// Receive Fragment Count
    receive_fragment_count: Reserved<u32>, // 0x40A8
    /// Receive Oversize Count
    receive_oversize_count: Reserved<u32>, // 0x40AC
    /// Receive Jabber Count
    receive_jabber_count: Reserved<u32>, // 0x40B0
    /// Management Packets Received Count
    management_packets_received_count: ReadOnly<u32>, // 0x40B4
    /// Management Packets Dropped Count
    management_packets_dropped_count: ReadOnly<u32>, // 0x40B8
    _padding86: [u8; 4], // 0x40BC - 0x40BF

    /// Total Octets Received
    total_octets_received: Reserved<u32>, // 0x40C0
    /// Total Octets Received
    total_octets_received: Reserved<u32>, // 0x40C4
    _padding88: [u8; 8], // 0x40C8 - 0x40CF

    /// Total Packets Received
    total_packets_received: Reserved<u32>, // 0x40D0
    /// Total Packets Transmitted
    total_packets_transmitted: Reserved<u32>, // 0x40D4
    /// Packets Transmitted Count 1
    packets_transmitted_count_1: Reserved<u32>, // 0x40D8
    /// Packets Transmitted Count 2
    packets_transmitted_count_2: Reserved<u32>, // 0x40DC
    /// Packets Transmitted Count 3
    packets_transmitted_count_3: Reserved<u32>, // 0x40E0
    /// Packets Transmitted Count 4
    packets_transmitted_count_4: Reserved<u32>, // 0x40E4
    /// Packets Transmitted Count 5
    packets_transmitted_count_5: Reserved<u32>, // 0x40E8
    _padding95: [u8; 4], // 0x40EC - 0x40EF

    /// Multicast Packets Transmitted
    multicast_packets_transmitted: Reserved<u32>, // 0x40F0
    /// Broadcast Packets Transmitted
    broadcast_packets_transmitted: Reserved<u32>, // 0x40F4
    _padding97: [u8; 40], // 0x40F8 - 0x411F

    /// XSUM Error Count
    xsum_error_count: Reserved<u32>, // 0x4120
    priority_xon_received_count: [Volatile<u32>; 8],
    priority_xoff_received_count: [Volatile<u32>; 8],
    _padding100: [u8; 220], // 0x4124 - 0x41FF

    /// PCS_1G Global Config Register 1
    pcs_1g_global_config_register_1: Volatile<u32>, // 0x4200
    _padding101: [u8; 4], // 0x4204 - 0x4207

    /// PCG_1G link Control Register
    pcg_1g_link_control_register: Volatile<u32>, // 0x4208
    /// PCS_1G Link Status Register
    pcs_1g_link_status_register: ReadOnly<u32>, // 0x420C
    _padding103: [u8; 8], // 0x4210 - 0x4217

    /// PCS_1 Gb/s Auto-Negotiation Advanced Register
    pcs_1_gb_s_auto_negotiation_advanced_register: Volatile<u32>, // 0x4218
    _padding104: [u8; 4], // 0x421C - 0x421F

    /// PCS_1GAN LP Ability Register
    pcs_1gan_lp_ability_register: ReadOnly<u32>, // 0x4220
    /// PCS_1G Auto-Negotiation Next Page Transmit Register
    pcs_1g_auto_negotiation_next_page_transmit_register: Volatile<u32>, // 0x4224
    _padding106: [u8; 24], // 0x4228 - 0x423F

    /// PCS_1G Auto-Negotiation LP's Next Page Register
    pcs_1g_auto_negotiation_lp's_next_page_register: ReadOnly<u32>, // 0x4240
    /// MAC Core Control 0 Register
    mac_core_control_0_register: Volatile<u32>, // 0x4244
    /// MAC Core Status 1 Register
    mac_core_status_1_register: ReadOnly<u32>, // 0x4248
    /// Pause and Pace Register
    pause_and_pace_register: Volatile<u32>, // 0x4248
    _padding110: [u8; 16], // 0x424C - 0x425B

    /// MDI Single Command and Address
    mdi_single_command_and_address: Volatile<u32>, // 0x425C
    /// MDI Single Read and Write Data
    mdi_single_read_and_write_data: Volatile<u32>, // 0x4260
    _padding112: [u8; 4], // 0x4264 - 0x4267

    /// Max Frame Size
    max_frame_size: Volatile<u32>, // 0x4268
    _padding113: [u8; 28], // 0x426C - 0x4287

    /// XGXS Status 1
    xgxs_status_1: ReadOnly<u32>, // 0x4288
    /// XGXS Status 2
    xgxs_status_2: ReadOnly<u32>, // 0x428C
    /// 10GBASE-X PCS Status
    10gbase_x_pcs_status: ReadOnly<u32>, // 0x4290
    /// MAC Flow Control Register
    mac_flow_control_register: Volatile<u32>, // 0x4294
    /// SerDes Interface Control Register
    serdes_interface_control_register: Volatile<u32>, // 0x4298
    /// FIFO Status/CNTL report Register
    fifo_status_cntl_report_register: Volatile<u32>, // 0x429C
    /// Auto-Negotiation Control Register
    auto_negotiation_control_register: Volatile<u32>, // 0x42A0
    /// Link Status Register
    link_status_register: ReadOnly<u32>, // 0x42A4
    /// Auto-Negotiation Control 2 Register
    auto_negotiation_control_2_register: Volatile<u32>, // 0x42A8
    _padding122: [u8; 8], // 0x42AC - 0x42B3

    /// Auto-Negotiation Link Partner Link Control Word 1 Register
    auto_negotiation_link_partner_link_control_word_1_register: ReadOnly<u32>, // 0x42B4
    _padding123: [u8; 24], // 0x42B8 - 0x42CF

    /// MAC Manageability Control Register
    mac_manageability_control_register: ReadOnly<u32>, // 0x42D0
    /// Auto-Negotiation Link Partner Next Page 1 register
    auto_negotiation_link_partner_next_page_1_register: ReadOnly<u32>, // 0x42D4
    /// Auto-Negotiation Link Partner Next Page 2 register
    auto_negotiation_link_partner_next_page_2_register: ReadOnly<u32>, // 0x42D8
    _padding126: [u8; 4], // 0x42DC - 0x42DF

    /// KR PCS and FEC Control Register
    kr_pcs_and_fec_control_register: Volatile<u32>, // 0x42E0
    /// KR PCS Status Register
    kr_pcs_status_register: ReadOnly<u32>, // 0x42E4
    /// FEC Status 1 Register
    fec_status_1_register: Reserved<u32>, // 0x42E8
    /// FEC Status 2 Register
    fec_status_2_register: Reserved<u32>, // 0x42EC
    _padding130: [u8; 36], // 0x42F0 - 0x4313

    /// SGMII Control Register
    sgmii_control_register: Volatile<u32>, // 0x4314
    _padding131: [u8; 4], // 0x4318 - 0x431B

    /// Priority Flow Control Type
    priority_flow_control_type: Volatile<u32>, // 0x431C
    _padding132: [u8; 4], // 0x4320 - 0x4323

    /// Link Status Register 2
    link_status_register_2: ReadOnly<u32>, // 0x4324
    _padding133: [u8; 1496], // 0x4328 - 0x48FF

    /// DCB Transmit Descriptor Plane Control and Status
    dcb_transmit_descriptor_plane_control_and_status: Volatile<u32>, // 0x4900
    /// DCB Transmit Descriptor Plane Queue Select
    dcb_transmit_descriptor_plane_queue_select: Volatile<u32>, // 0x4904
    /// DCB Transmit Descriptor Plane T1 Config
    dcb_transmit_descriptor_plane_t1_config: Volatile<u32>, // 0x4908
    dcb_transmit_descriptor_plane_t2_config: [Volatile<u32>; 8],
    tx_packet_buffer_threshold: [Volatile<u32>; 8],
    _padding138: [u8; 116], // 0x490C - 0x497F

    /// DCB Transmit Rate–Scheduler MMW
    dcb_transmit_rate–scheduler_mmw: Volatile<u32>, // 0x4980
    _padding139: [u8; 252], // 0x4984 - 0x4A7F

    /// DMA Tx Control
    dma_tx_control: Volatile<u32>, // 0x4A80
    _padding140: [u8; 4], // 0x4A84 - 0x4A87

    /// DMA Tx TCP Flags Control Low
    dma_tx_tcp_flags_control_low: Volatile<u32>, // 0x4A88
    /// DMA Tx TCP Flags Control High
    dma_tx_tcp_flags_control_high: Volatile<u32>, // 0x4A8C
    pf_mailbox_target: [Volatile<u32>; 64],
    _padding143: [u8; 1392], // 0x4A90 - 0x4FFF

    /// Receive Checksum Control
    receive_checksum_control: Volatile<u32>, // 0x5000
    _padding144: [u8; 4], // 0x5004 - 0x5007

    /// Receive Filter Control Register
    receive_filter_control_register: Volatile<u32>, // 0x5008
    management_vlan_tag_value: [Volatile<u32>; 8],
    management_flex_udp_tcp_ports: [Volatile<u32>; 8],
    _padding147: [u8; 108], // 0x500C - 0x5077

    /// Extended VLAN Ether Type
    extended_vlan_ether_type: Volatile<u32>, // 0x5078
    _padding148: [u8; 4], // 0x507C - 0x507F

    /// Filter Control Register
    filter_control_register: Volatile<u32>, // 0x5080
    _padding149: [u8; 4], // 0x5084 - 0x5087

    /// VLAN Control Register
    vlan_control_register: Volatile<u32>, // 0x5088
    _padding150: [u8; 4], // 0x508C - 0x508F

    /// Multicast Control Register
    multicast_control_register: Volatile<u32>, // 0x5090
    _padding151: [u8; 108], // 0x5094 - 0x50FF

    /// FC Receive Control
    fc_receive_control: Volatile<u32>, // 0x5100
    _padding152: [u8; 4], // 0x5104 - 0x5107

    /// FC FLT Context
    fc_flt_context: Volatile<u32>, // 0x5108
    _padding153: [u8; 4], // 0x510C - 0x510F

    /// FC Filter RW Control
    fc_filter_rw_control: WriteOnly<u32>, // 0x5110
    _padding154: [u8; 12], // 0x5114 - 0x511F

    /// Rx Message Type Register Low
    rx_message_type_register_low: Volatile<u32>, // 0x5120
    etq_filter: [Volatile<u32>; 8],
    manageability_decision_filters: [Volatile<u32>; 8],
    pf_vm_tx_switch_loopback_enable: [Volatile<u32>; 2],
    _padding158: [u8; 100], // 0x5124 - 0x5187

    /// Rx Time Sync Control Register
    rx_time_sync_control_register: Volatile<u32>, // 0x5188
    management_ethernet_type_filters: [Volatile<u32>; 4],
    _padding160: [u8; 20], // 0x518C - 0x519F

    /// Rx Timestamp Attributes Low
    rx_timestamp_attributes_low: ReadOnly<u32>, // 0x51A0
    /// Rx Timestamp High
    rx_timestamp_high: ReadOnly<u32>, // 0x51A4
    /// Rx Timestamp Attributes High
    rx_timestamp_attributes_high: ReadOnly<u32>, // 0x51A8
    _padding163: [u8; 4], // 0x51AC - 0x51AF

    /// PF Virtual Control Register
    pf_virtual_control_register: Volatile<u32>, // 0x51B0
    _padding164: [u8; 36], // 0x51B4 - 0x51D7

    /// FC Offset Parameter
    fc_offset_parameter: Volatile<u32>, // 0x51D8
    pf_vf_receive_enable: [Volatile<u32>; 1],
    _padding166: [u8; 12], // 0x51DC - 0x51E7

    /// Rx Timestamp Low
    rx_timestamp_low: ReadOnly<u32>, // 0x51E8
    multicast_table_array: [Volatile<u32>; 128],
    packet_split_receive_type_register: [Volatile<u32>; 16],
    _padding169: [u8; 1556], // 0x51EC - 0x57FF

    /// Wake Up Control Register
    wake_up_control_register: Volatile<u32>, // 0x5800
    _padding170: [u8; 4], // 0x5804 - 0x5807

    /// Wake Up Filter Control Register
    wake_up_filter_control_register: Volatile<u32>, // 0x5808
    _padding171: [u8; 20], // 0x580C - 0x581F

    /// Management Control Register
    management_control_register: Volatile<u32>, // 0x5820
    /// Manageability Filters Valid
    manageability_filters_valid: Volatile<u32>, // 0x5824
    _padding173: [u8; 16], // 0x5828 - 0x5837

    /// IP Address Valid
    ip_address_valid: Volatile<u32>, // 0x5838
    ipv4_address_table: [Volatile<u32>; 4],
    _padding175: [u8; 36], // 0x583C - 0x585F

    /// Management Control To Host Register
    management_control_to_host_register: Volatile<u32>, // 0x5860
    ipv6_address_table: [Volatile<u32>; 4],
    manageability_decision_filters: [Volatile<u32>; 8],
    manageability_ip_address_filter: [Volatile<u32>; 4],
    _padding179: [u8; 156], // 0x5864 - 0x58FF

    /// Wake Up Packet Length
    wake_up_packet_length: ReadOnly<u32>, // 0x5900
    manageability_ethernet_mac_address_high: [Volatile<u32>; 4],
    wake_up_packet_memory: [ReadOnly<u32>; 32],
    transmit_descriptor_base_address_low: [Volatile<u32>; 128],
    transmit_descriptor_base_address_high: [Volatile<u32>; 128],
    transmit_descriptor_length: [Volatile<u32>; 128],
    tx_dca_control_register: [Volatile<u32>; 128],
    dma_tx: [Volatile<u32>; 128],
    transmit_descriptor_head: [ReadOnly<u32>; 128],
    transmit_descriptor_tail: [Volatile<u32>; 128],
    queue_packets_transmitted_count: [RC<u32>; 16],
    transmit_descriptor_control: [Volatile<u32>; 128],
    tx_descriptor_completion_write_back_address_low: [Volatile<u32>; 128],
    tx_descriptor_completion_write_back_address_high: [Volatile<u32>; 128],
    transmit_queue_statistic_mapping_registers: [Volatile<u32>; 8],
    pf_vm_vlan_insert_register: [Volatile<u32>; 64],
    _padding195: [u8; 10236], // 0x5904 - 0x80FF

    /// DMA Tx TCP Max Allow Size Requests
    dma_tx_tcp_max_allow_size_requests: Volatile<u32>, // 0x8100
    pf_vf_transmit_enable: [Volatile<u32>; 1],
    _padding197: [u8; 28], // 0x8104 - 0x811F

    /// Multiple Transmit Queues Command Register
    multiple_transmit_queues_command_register: Volatile<u32>, // 0x8120
    pf_vf_anti_spoof_control: [Volatile<u32>; 8],
    _padding199: [u8; 252], // 0x8124 - 0x821F

    /// PF DMA Tx General Switch Control
    pf_dma_tx_general_switch_control: Volatile<u32>, // 0x8220
    strict_low_latency_tx_queues: [Volatile<u32>; 4],
    transmit_queue_statistic_mapping_registers: [Volatile<u32>; 32],
    queue_packets_transmitted_count: [RC<u32>; 16],
    queue_bytes_transmitted_count_low: [Volatile<u32>; 16],
    queue_bytes_transmitted_count_high: [RC<u32>; 16],
    _padding205: [u8; 1376], // 0x8224 - 0x8783

    /// FCoE Packets Transmitted Count
    fcoe_packets_transmitted_count: Reserved<u32>, // 0x8784
    /// FCoE DWord Transmitted Count
    fcoe_dword_transmitted_count: Reserved<u32>, // 0x8788
    _padding207: [u8; 20], // 0x878C - 0x879F

    /// DMA Good Tx Packet Counter
    dma_good_tx_packet_counter: Reserved<u32>, // 0x87A0
    /// DMA Good Tx Byte Counter Low
    dma_good_tx_byte_counter_low: Reserved<u32>, // 0x87A4
    /// DMA Good Tx Byte Counter High
    dma_good_tx_byte_counter_high: Reserved<u32>, // 0x87A8
    _padding210: [u8; 84], // 0x87AC - 0x87FF

    /// Security Tx Control
    security_tx_control: Volatile<u32>, // 0x8800
    /// Security Tx Status
    security_tx_status: ReadOnly<u32>, // 0x8804
    /// Security Tx Buffer Almost Full
    security_tx_buffer_almost_full: Volatile<u32>, // 0x8808
    _padding213: [u8; 244], // 0x880C - 0x88FF

    /// IPsec Tx Index
    ipsec_tx_index: Volatile<u32>, // 0x8900
    /// IPsec Tx Salt Register
    ipsec_tx_salt_register: Volatile<u32>, // 0x8904
    ipsec_tx_key_registers: [Volatile<u32>; 4],
    _padding216: [u8; 248], // 0x8908 - 0x89FF

    /// LinkSec Tx Capabilities Register
    linksec_tx_capabilities_register: Volatile<u32>, // 0x8A00
    /// LinkSec Tx Control Register
    linksec_tx_control_register: Volatile<u32>, // 0x8A04
    /// LinkSec Tx SCI Low
    linksec_tx_sci_low: Volatile<u32>, // 0x8A08
    /// LinkSec Tx SCI High
    linksec_tx_sci_high: ReadOnly<u32>, // 0x8A0C
    /// LinkSec Tx SA
    linksec_tx_sa: Volatile<u32>, // 0x8A10
    /// LinkSec Tx SA PN 0
    linksec_tx_sa_pn_0: Volatile<u32>, // 0x8A14
    /// LinkSec Tx SA PN 1
    linksec_tx_sa_pn_1: Volatile<u32>, // 0x8A18
    linksec_tx_key_0: [WriteOnly<u32>; 4],
    linksec_tx_key_1: [WriteOnly<u32>; 4],
    _padding225: [u8; 32], // 0x8A1C - 0x8A3B

    /// Tx Untagged Packet Counter
    tx_untagged_packet_counter: ReadOnly<u32>, // 0x8A3C
    /// Encrypted Tx Packets
    encrypted_tx_packets: ReadOnly<u32>, // 0x8A40
    /// Protected Tx Packets
    protected_tx_packets: ReadOnly<u32>, // 0x8A44
    /// Encrypted Tx Octets
    encrypted_tx_octets: ReadOnly<u32>, // 0x8A48
    /// Protected Tx Octets
    protected_tx_octets: ReadOnly<u32>, // 0x8A4C
    _padding230: [u8; 432], // 0x8A50 - 0x8BFF

    /// Tx Time Sync Control Register
    tx_time_sync_control_register: Volatile<u32>, // 0x8C00
    /// Tx Timestamp Value Low
    tx_timestamp_value_low: ReadOnly<u32>, // 0x8C04
    /// Tx Timestamp Value High
    tx_timestamp_value_high: ReadOnly<u32>, // 0x8C08
    /// System Time
    system_time: Volatile<u32>, // 0x8C0C
    /// System Time Register
    system_time_register: Volatile<u32>, // 0x8C10
    /// Increment Attributes Register
    increment_attributes_register: Volatile<u32>, // 0x8C14
    /// Time Adjustment Offset Register low
    time_adjustment_offset_register_low: Volatile<u32>, // 0x8C18
    /// Time Adjustment Offset Register High
    time_adjustment_offset_register_high: Volatile<u32>, // 0x8C1C
    /// TimeSync Auxiliary Control Register
    timesync_auxiliary_control_register: Volatile<u32>, // 0x8C20
    /// Target Time Register 0 Low
    target_time_register_0_low: Volatile<u32>, // 0x8C24
    /// Target Time Register 0 High
    target_time_register_0_high: Volatile<u32>, // 0x8C28
    /// Target Time Register 1 Low
    target_time_register_1_low: Volatile<u32>, // 0x8C2C
    /// Target Time Register 1 High
    target_time_register_1_high: Volatile<u32>, // 0x8C30
    _padding243: [u8; 8], // 0x8C34 - 0x8C3B

    /// Auxiliary Time Stamp 0 Register low
    auxiliary_time_stamp_0_register_low: ReadOnly<u32>, // 0x8C3C
    /// Auxiliary Time Stamp 0 Register high
    auxiliary_time_stamp_0_register_high: ReadOnly<u32>, // 0x8C40
    /// Auxiliary Time Stamp 1 Register low
    auxiliary_time_stamp_1_register_low: ReadOnly<u32>, // 0x8C44
    /// Auxiliary Time Stamp 1
    auxiliary_time_stamp_1: ReadOnly<u32>, // 0x8C48
    _padding247: [u8; 180], // 0x8C4C - 0x8CFF

    /// Security Rx Control
    security_rx_control: Volatile<u32>, // 0x8D00
    /// Security Rx Status
    security_rx_status: ReadOnly<u32>, // 0x8D04
    _padding249: [u8; 248], // 0x8D08 - 0x8DFF

    /// IPsec Rx Index
    ipsec_rx_index: Volatile<u32>, // 0x8E00
    ipsec_rx_ip_address_register: [Volatile<u32>; 4],
    _padding251: [u8; 16], // 0x8E04 - 0x8E13

    /// IPsec Rx SPI Register
    ipsec_rx_spi_register: Volatile<u32>, // 0x8E14
    /// IPsec Rx SPI Register
    ipsec_rx_spi_register: Volatile<u32>, // 0x8E18
    _padding253: [u8; 16], // 0x8E1C - 0x8E2B

    /// IPsec Rx Salt Register
    ipsec_rx_salt_register: Volatile<u32>, // 0x8E2C
    /// IPsec Rx Mode Register
    ipsec_rx_mode_register: Volatile<u32>, // 0x8E30
    _padding255: [u8; 204], // 0x8E34 - 0x8EFF

    /// LinkSec Rx Capabilities Register
    linksec_rx_capabilities_register: Volatile<u32>, // 0x8F00
    /// LinkSec Rx Control Register
    linksec_rx_control_register: Volatile<u32>, // 0x8F04
    /// LinkSec Rx SCI Low
    linksec_rx_sci_low: Volatile<u32>, // 0x8F08
    /// LinkSec Rx SCI High
    linksec_rx_sci_high: Volatile<u32>, // 0x8F0C
    linksec_rx_sa: [Volatile<u32>; 2],
    linksec_rx_sa_pn: [Volatile<u32>; 2],
    m]: [Volatile<u32>; n],
    _padding262: [u8; 48], // 0x8F10 - 0x8F3F

    /// LinkSec Untagged Rx Packet
    linksec_untagged_rx_packet: ReadOnly<u32>, // 0x8F40
    /// LinkSec Rx Octets Decrypted
    linksec_rx_octets_decrypted: ReadOnly<u32>, // 0x8F44
    /// LinkSec Rx Octets Validated
    linksec_rx_octets_validated: ReadOnly<u32>, // 0x8F48
    /// LinkSec Rx Packet with Bad Tag
    linksec_rx_packet_with_bad_tag: ReadOnly<u32>, // 0x8F4C
    /// LinkSec No SCI
    linksec_no_sci: ReadOnly<u32>, // 0x8F50
    /// LinkSec Unknown SCI
    linksec_unknown_sci: ReadOnly<u32>, // 0x8F54
    /// LinkSec Rx Unchecked Packets
    linksec_rx_unchecked_packets: ReadOnly<u32>, // 0x8F58
    _padding269: [u8; 4], // 0x8F5C - 0x8F5F

    /// LinkSec Rx Late Packets
    linksec_rx_late_packets: ReadOnly<u32>, // 0x8F60
    linksec_rx_packet_ok: [ReadOnly<u32>; n],
    linksec_rx_invalid: [ReadOnly<u32>; n],
    linksec_rx_not_valid: [ReadOnly<u32>; 2],
    _padding273: [u8; 24], // 0x8F64 - 0x8F7B

    /// LinkSec Rx Unused SA
    linksec_rx_unused_sa: Reserved<u32>, // 0x8F7C
    /// LinkSec Rx Not Using SA
    linksec_rx_not_using_sa: Reserved<u32>, // 0x8F80
    flexible_host_filter_table_registers: [Volatile<u32>; 5],
    flexible_tco_filter_tableregisters: [Volatile<u32>; 3],
    vlan_filter_table_array: [Volatile<u32>; 128],
    receive_address_low: [Volatile<u32>; 128],
    receive_address_high: [Volatile<u32>; 128],
    _padding280: [u8; 14460], // 0x8F84 - 0xC7FF

    /// DCB Transmit User Priority to Traffic Class
    dcb_transmit_user_priority_to_traffic_class: Volatile<u32>, // 0xC800
    transmit_packet_buffer_size: [Volatile<u32>; 8],
    _padding282: [u8; 1276], // 0xC804 - 0xCCFF

    /// DCB Transmit Packet Plane Control and Status
    dcb_transmit_packet_plane_control_and_status: Volatile<u32>, // 0xCD00
    _padding283: [u8; 12], // 0xCD04 - 0xCD0F

    /// Manageability Transmit TC Mapping
    manageability_transmit_tc_mapping: Volatile<u32>, // 0xCD10
    dcb_transmit_packet_plane_t2_config: [Volatile<u32>; 8],
    dcb_transmit_packet_plane_t2_status: [ReadOnly<u32>; 8],
    _padding286: [u8; 236], // 0xCD14 - 0xCDFF

    /// Transmit Flow Control Status
    transmit_flow_control_status: ReadOnly<u32>, // 0xCE00
    source_address_queue_filter: [Volatile<u32>; 128],
    destination_address_queue_filter: [Volatile<u32>; 128],
    source_destination_port_queue_filter: [Volatile<u32>; 128],
    five_tuple_queue_filter: [Volatile<u32>; 128],
    l3_l4_tuples_immediate_interrupt: [Volatile<u32>; 128],
    ipsec_rx_key_register: [Volatile<u32>; 4],
    redirection_table: [Volatile<u32>; 32],
    rss_random_key_register: [Volatile<u32>; 10],
    e_type_queue_select: [Volatile<u32>; 8],
    _padding296: [u8; 7724], // 0xCE04 - 0xEC2F

    /// SYN Packet Queue Filter
    syn_packet_queue_filter: Volatile<u32>, // 0xEC30
    _padding297: [u8; 44], // 0xEC34 - 0xEC5F

    /// Immediate Interrupt Rx VLAN Priority Register
    immediate_interrupt_rx_vlan_priority_register: Volatile<u32>, // 0xEC60
    _padding298: [u8; 12], // 0xEC64 - 0xEC6F

    /// RSS Queues Per Traffic Class Register
    rss_queues_per_traffic_class_register: Volatile<u32>, // 0xEC70
    _padding299: [u8; 28], // 0xEC74 - 0xEC8F

    /// LLI Size Threshold
    lli_size_threshold: Volatile<u32>, // 0xEC90
    _padding300: [u8; 108], // 0xEC94 - 0xECFF

    /// FCoE Redirection Control
    fcoe_redirection_control: Volatile<u32>, // 0xED00
    fc_oe_redirection_table: [Volatile<u32>; 8],
    _padding302: [u8; 252], // 0xED04 - 0xEDFF

    /// Flow Director Filters Control Register
    flow_director_filters_control_register: Volatile<u32>, // 0xEE00
    flow_director_filters_source_ipv6: [Volatile<u32>; 3],
    _padding304: [u8; 20], // 0xEE04 - 0xEE17

    /// Flow Director Filters IP SA
    flow_director_filters_ip_sa: Volatile<u32>, // 0xEE18
    /// Flow Director Filters IP DA
    flow_director_filters_ip_da: Volatile<u32>, // 0xEE1C
    /// Flow Director Filters Port
    flow_director_filters_port: Volatile<u32>, // 0xEE20
    /// Flow Director Filters VLAN and FLEX bytes
    flow_director_filters_vlan_and_flex_bytes: Volatile<u32>, // 0xEE24
    /// Flow Director Filters Hash Signature
    flow_director_filters_hash_signature: Volatile<u32>, // 0xEE28
    /// Flow Director Filters Command Register
    flow_director_filters_command_register: Volatile<u32>, // 0xEE2C
    _padding310: [u8; 8], // 0xEE30 - 0xEE37

    /// Flow Director Filters Free
    flow_director_filters_free: Volatile<u32>, // 0xEE38
    /// Flow Director Filters IPv4 Mask
    flow_director_filters_ipv4_mask: Volatile<u32>, // 0xEE3C
    /// Flow Director Filters Source IPv4 Mask
    flow_director_filters_source_ipv4_mask: Volatile<u32>, // 0xEE40
    /// Flow Director Filters TCP Mask
    flow_director_filters_tcp_mask: Volatile<u32>, // 0xEE44
    /// Flow Director Filters UDP Mask
    flow_director_filters_udp_mask: Volatile<u32>, // 0xEE48
    /// Flow Director Filters Length
    flow_director_filters_length: Reserved<u32>, // 0xEE4C
    /// Flow Director Filters Usage Statistics
    flow_director_filters_usage_statistics: Reserved<u32>, // 0xEE50
    /// Flow Director Filters Failed Usage Statistics
    flow_director_filters_failed_usage_statistics: Reserved<u32>, // 0xEE54
    /// Flow Director Filters Match Statistics
    flow_director_filters_match_statistics: Reserved<u32>, // 0xEE58
    _padding319: [u8; 12], // 0xEE5C - 0xEE67

    /// Flow Director Filters Lookup Table Hash Key
    flow_director_filters_lookup_table_hash_key: Volatile<u32>, // 0xEE68
    /// Flow Director Filters Lookup Table Stream Key
    flow_director_filters_lookup_table_stream_key: Volatile<u32>, // 0xEE6C
    /// Flow Director Filters Other Mask
    flow_director_filters_other_mask: Volatile<u32>, // 0xEE70
    /// Flow Director Filters IPv6 Mask
    flow_director_filters_ipv6_mask: Volatile<u32>, // 0xEE74
    pf_vm_l2_control_register: [Volatile<u32>; 64],
    pf: [Volatile<u32>; 64],
    pf: [Volatile<u32>; 128],
    pf: [Volatile<u32>; 128],
    pf: [Volatile<u32>; 4],
    pf: [Volatile<u32>; 8],
    pf: [Volatile<u32>; 8],
    _padding330: [u8; 4504], // 0xEE78 - 0x1000F

    /// EEPROM/Flash Control Register
    eeprom_flash_control_register: Volatile<u32>, // 0x10010
    /// EEPROM Read Register
    eeprom_read_register: Volatile<u32>, // 0x10014
    _padding332: [u8; 4], // 0x10018 - 0x1001B

    /// Flash Access Register
    flash_access_register: Volatile<u32>, // 0x1001C
    _padding333: [u8; 244], // 0x10020 - 0x10113

    /// Manageability EEPROM Read/Write Data
    manageability_eeprom_read_write_data: Volatile<u32>, // 0x10114
    /// Manageability Flash Control Register
    manageability_flash_control_register: Volatile<u32>, // 0x10118
    /// Manageability Flash Read Data
    manageability_flash_read_data: Volatile<u32>, // 0x1011C
    _padding336: [u8; 32], // 0x10120 - 0x1013F

    /// Software Semaphore Register
    software_semaphore_register: Volatile<u32>, // 0x10140
    _padding337: [u8; 4], // 0x10144 - 0x10147

    /// Firmware Semaphore Register
    firmware_semaphore_register: Volatile<u32>, // 0x10148
    _padding338: [u8; 4], // 0x1014C - 0x1014F

    /// Function Active and Power State to Manageability
    function_active_and_power_state_to_manageability: ReadOnly<u32>, // 0x10150
    _padding339: [u8; 12], // 0x10154 - 0x1015F

    /// Software–Firmware Synchronization
    software–firmware_synchronization: Volatile<u32>, // 0x10160
    _padding340: [u8; 3740], // 0x10164 - 0x10FFF

    /// PCIe Control Register
    pcie_control_register: Volatile<u32>, // 0x11000
    _padding341: [u8; 12], // 0x11004 - 0x1100F

    /// PCIe Statistic Control Register 1
    pcie_statistic_control_register_1: Volatile<u32>, // 0x11010
    /// PCIe Statistic Control Registers 2
    pcie_statistic_control_registers_2: Volatile<u32>, // 0x11014
    pcie_statistic_counter_registers: [ReadOnly<u32>; 4],
    pcie_statistic_control_register: [Volatile<u32>; 4],
    _padding345: [u8; 40], // 0x11018 - 0x1103F

    /// PCIe PHY Address Register
    pcie_phy_address_register: Volatile<u32>, // 0x11040
    /// PCIe PHY Data Register
    pcie_phy_data_register: Volatile<u32>, // 0x11044
    _padding347: [u8; 8], // 0x11048 - 0x1104F

    /// PCIe Control Extended Register
    pcie_control_extended_register: Volatile<u32>, // 0x11050
    _padding348: [u8; 16], // 0x11054 - 0x11063

    /// Mirrored Revision ID
    mirrored_revision_id: ReadOnly<u32>, // 0x11064
    _padding349: [u8; 8], // 0x11068 - 0x1106F

    /// DCA Requester ID Information Register
    dca_requester_id_information_register: ReadOnly<u32>, // 0x11070
    /// DCA Control Register
    dca_control_register: Volatile<u32>, // 0x11074
    _padding351: [u8; 56], // 0x11078 - 0x110AF

    /// PCIe Interrupt Cause
    pcie_interrupt_cause: ReadOnly<u32>, // 0x110B0
    _padding352: [u8; 4], // 0x110B4 - 0x110B7

    /// PCIe Interrupts Enable
    pcie_interrupts_enable: Volatile<u32>, // 0x110B8
    msi_x_pba_clear: [Volatile<u32>; 8],
    extended_interrupt_throttle: [Volatile<u32>; 5],
    _padding355: [u8; 15940], // 0x110BC - 0x14EFF

    /// Core Analog Configuration Register
    core_analog_configuration_register: Volatile<u32>, // 0x14F00
    _padding356: [u8; 12], // 0x14F04 - 0x14F0F

    /// Core Common Configuration Register
    core_common_configuration_register: Volatile<u32>, // 0x14F10
    _padding357: [u8; 4096], // 0x14F14 - 0x15F13

    /// LinkSec SW/FW Interface MNG
    linksec_sw_fw_interface_mng: Reserved<u32>, // 0x15F14
}
