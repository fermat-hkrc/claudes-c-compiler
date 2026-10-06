struct SD { float a; double b; float c; double d; };  // 32B mixed float struct
double s(struct SD v) { return v.a + v.b + v.c + v.d; }
int main(void) { struct SD v = { 1.5f, 2.25, 3.75f, 4.5 }; printf("%.17g\n", s(v)); return 0; }
