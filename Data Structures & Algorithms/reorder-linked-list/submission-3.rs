// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut curr=head.as_ref();
        let mut len=0;
        while let Some(node)=curr{
            len+=1;
            curr=node.next.as_ref();
        }
        let mut curr=head.as_mut();
        for _ in 0..len/2{
            curr=curr.unwrap().next.as_mut();
        }
        let mut curr=curr.unwrap().next.take();
        let mut prev=None;
        while let Some(mut node)=curr{
            let after=node.next.take();
            node.next=prev;
            prev=Some(node);
            curr=after;
        }
        let mut first_half=head.as_mut();
        let mut reversed_second=prev;
        while let Some(node1)=first_half{
            if let Some(mut node2)=reversed_second{
                let next_first=node1.next.take();
                reversed_second=node2.next.take();
                node2.next=next_first;
                node1.next=Some(node2);
            }else{
                break;
            }
            first_half=node1.next.as_mut().unwrap().next.as_mut();
        }
    }
}
