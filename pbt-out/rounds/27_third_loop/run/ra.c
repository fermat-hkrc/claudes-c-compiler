extern int __attribute__((weak)) optional_fn(void);
extern int strong_fn(void);
extern int shared_counter;
int call_all(void) {
    int result = strong_fn();
    if (optional_fn) result += optional_fn();  // weak: may be null
    return result + shared_counter;
}
