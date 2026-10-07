static int f1(int *p, int *q) { *p += 843; return *q * 2; }
static int f2(int *p, int *q) { *p += 619; return *q * 2; }
static int f3(int *p, int *q) { *p += 910; return *q * 2; }
static int f4(int *p, int *q) { *p += 373; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    sum += f1(&cells[0], &cells[6]);
    sum += f2(&cells[1], &cells[6]);
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += f3(&cells[1], &cells[7]);
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    sum += f4(&cells[7], &cells[6]);
    arr[1 % 6] = 405; cells[3] += arr[1 % 6];
    arr[2 % 6] = 199; cells[5] += arr[2 % 6];
    arr[2 % 6] = 569; cells[4] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }