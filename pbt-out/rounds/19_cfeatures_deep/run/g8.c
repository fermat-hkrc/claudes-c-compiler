struct Node { int v; struct Node *next; };
int main(void) { struct Node c = { 3, 0 }, b = { 2, &c }, a = { 1, &b };
  int t = 0; for (struct Node *p = &a; p; p = p->next) t = t * 10 + p->v;
  printf("%d\n", t); return 0; }
