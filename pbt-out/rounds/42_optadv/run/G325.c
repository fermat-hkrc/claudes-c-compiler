static int f1(int *p, int *q) { *p += 221; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[7]);
    cells[0] = cells[2] + 866;
    cells[1] = cells[1] + 635;
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 360u; }
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 15u; }
    cells[6] = cells[3] + 457;
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 523u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    cells[1] = cells[5] + 175;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }