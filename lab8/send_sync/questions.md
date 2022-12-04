1) I saw someone's code fail to compile because they 
were trying to send non-thread-safe data across threads. 
How does the Rust language allow for static (i.e. at compile time)
guarantees that specific data can be sent/shared acrosss threads?

2) Do you have to then implement the Send and Sync traits for 
every piece of data (i.e. a struct) you want to share and send across threads?

3) What types in the course have I seen that aren't Send? Give one example, 
and explain why that type isn't Send 

4) What is the relationship between Send and Sync? Does this relate
to Rust's Ownership system somehow?

5) Are there any types that could be Send but NOT Sync? Is that even possible?

6) Could we implement Send ourselves using safe rust? why/why not?
