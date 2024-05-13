use std::{collections::LinkedList, sync::Mutex};

use crate::device::{DeviceError, ToCardWorkRbDesc};

use crate::types::Qpn;

use super::SchedulerStrategy;

/// The round-robin strategy for the scheduler.
#[allow(clippy::module_name_repetitions, clippy::linkedlist)]
#[derive(Debug)]
pub(crate) struct RoundRobinStrategy {
    queue: Mutex<LinkedList<(u32, LinkedList<ToCardWorkRbDesc>)>>,
}

impl RoundRobinStrategy {
    pub(crate) fn new() -> Self {
        Self {
            queue: Mutex::new(LinkedList::new()),
        }
    }
}

impl Default for RoundRobinStrategy {
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: SchedulerStrategy should guarantee that it is thread safe
unsafe impl Send for RoundRobinStrategy {}
unsafe impl Sync for RoundRobinStrategy {}

impl SchedulerStrategy for RoundRobinStrategy {
    fn push(&self, qpn: Qpn, desc: LinkedList<ToCardWorkRbDesc>) -> Result<(), DeviceError> {
        for i in self
            .queue
            .lock()
            .map_err(|_| DeviceError::LockPoisoned("scheduler queue lock".to_owned()))?
            .iter_mut()
        {
            // merge the descriptor if the qpn is already in the queue
            if i.0 == qpn.get() {
                i.1.extend(desc);
                return Ok(());
            }
        }

        self.queue
            .lock()
            .map_err(|_| DeviceError::LockPoisoned("scheduler queue lock".to_owned()))?
            .push_back((qpn.get(), desc));
        Ok(())
    }

    #[allow(clippy::unwrap_in_result, clippy::unwrap_used)]
    fn pop(&self) -> Result<Option<ToCardWorkRbDesc>, DeviceError> {
        let mut guard = self
            .queue
            .lock()
            .map_err(|_| DeviceError::LockPoisoned("scheduler queue lock".to_owned()))?;

        let desc = if let Some((_, list)) = guard.front_mut() {
            // the front_mut is existed,so the pop_front will not return None
            list.pop_front().unwrap()
        } else {
            // otherwise it will return None
            return Ok(None);
        };

        // the front_mut is existed,so the pop_front will not return None
        let (qpn, list) = guard.pop_front().unwrap();
        if !list.is_empty() {
            guard.push_back((qpn, list));
        }
        Ok(Some(desc))
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::LinkedList, net::Ipv4Addr};

    use eui48::MacAddress;

    use crate::{
        device::{
            scheduler::{
                get_to_card_desc_common, round_robin::RoundRobinStrategy, SchedulerStrategy,
            },
            ToCardCtrlRbDescSge, ToCardWorkRbDesc, ToCardWorkRbDescCommon, ToCardWorkRbDescWrite,
        },
        types::{Key, MemAccessTypeFlag, Msn, Pmtu, Psn, QpType, Qpn},
    };

    pub(crate) fn generate_random_descriptors(
        qpn: u32,
        num: usize,
    ) -> LinkedList<ToCardWorkRbDesc> {
        let desc = ToCardWorkRbDesc::Write(ToCardWorkRbDescWrite {
            common: ToCardWorkRbDescCommon {
                total_len: 512,
                raddr: 0x0,
                rkey: Key::new(1234_u32),
                dqp_ip: Ipv4Addr::new(127, 0, 0, 1),
                dqpn: Qpn::new(qpn),
                mac_addr: MacAddress::default(),
                pmtu: Pmtu::Mtu1024,
                flags: MemAccessTypeFlag::IbvAccessNoFlags,
                qp_type: QpType::Rc,
                psn: Psn::new(1234),
                msn: Msn::new(0),
            },
            is_last: true,
            is_first: true,
            sge0: ToCardCtrlRbDescSge {
                addr: 0x1000,
                len: 512,
                key: Key::new(0x1234_u32),
            },
            sge1: None,
            sge2: None,
            sge3: None,
        });
        let mut ret = LinkedList::new();
        for _ in 0..num {
            ret.push_back(desc.clone());
        }
        ret
    }

    #[test]
    fn test_round_robin() {
        let round_robin = RoundRobinStrategy::new();
        let qpn1 = Qpn::new(1);
        let qpn2 = Qpn::new(2);
        let qpn1_descs = generate_random_descriptors(1, 2);
        round_robin.push(qpn1, qpn1_descs).unwrap();
        let qpn2_descs = generate_random_descriptors(2, 3);
        round_robin.push(qpn2, qpn2_descs).unwrap();
        let result_dqpns = [1, 2, 1, 2, 2];
        for result_dqpn in result_dqpns {
            let desc = round_robin.pop().unwrap().unwrap();
            let item = get_to_card_desc_common(&desc).dqpn;
            assert_eq!(item.get(), result_dqpn);
        }

        // test merge descriptors
        let qpn1_descs = generate_random_descriptors(1, 2);
        round_robin.push(qpn1, qpn1_descs).unwrap();
        let qpn2_descs = generate_random_descriptors(2, 3);
        round_robin.push(qpn2, qpn2_descs).unwrap();
        let desc = round_robin.pop().unwrap().unwrap();
        let item1 = get_to_card_desc_common(&desc).dqpn;
        assert_eq!(item1.get(), 1);
        // should be {qpn1 : 3 items, qpn2 : 3 items}, next is qpn2
        let qpn1_descs = generate_random_descriptors(1, 2);
        round_robin.push(qpn1, qpn1_descs).unwrap();
        let result_dqpns = [2, 1, 2, 1, 2, 1];
        for result_dqpn in result_dqpns {
            let desc = round_robin.pop().unwrap().unwrap();
            let item = get_to_card_desc_common(&desc).dqpn;
            assert_eq!(item.get(), result_dqpn);
        }
    }
}
