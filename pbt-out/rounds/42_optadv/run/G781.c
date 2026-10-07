static int f1(int *p, int *q) { *p += 873; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 375u; }
    arr[2 % 6] = 422; cells[2] += arr[2 % 6];
    cells[6] = cells[1] + 279;
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    cells[6] = cells[4] + 572;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    arr[1 % 6] = 530; cells[2] += arr[1 % 6];
    cells[6] = cells[0] + 311;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    cells[0] = cells[5] + 28;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 876u; }
    cells[5] = cells[2] + 528;
    arr[6 % 6] = 160; cells[0] += arr[6 % 6];
    sum += f1(&cells[1], &cells[4]);
    arr[7 % 6] = 572; cells[4] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }