#define F(x) G(x)
#define G(x) F(x)
int a = F(1);
#undef F
#define F(x) (x)+1
int b = F(2);
