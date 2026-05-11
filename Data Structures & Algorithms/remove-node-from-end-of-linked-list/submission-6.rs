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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len=0;
        let mut head=head;
        let mut curr=head.as_ref();
        while let Some(node)=curr{
            len+=1;
            curr=node.next.as_ref();
        }
        if len<=n{
            return head.unwrap().next;
        }
        let mut curr_mut = head.as_mut();
        for _ in 0..(len - n - 1) {
            curr_mut = curr_mut.unwrap().next.as_mut();
        }
        let mut node_to_remove = curr_mut.as_mut().unwrap().next.take();
        curr_mut.unwrap().next = node_to_remove.unwrap().next.take();
        return head;
        
    }
}
