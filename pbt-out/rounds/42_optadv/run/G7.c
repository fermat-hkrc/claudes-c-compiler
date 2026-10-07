static int f1(int *p, int *q) { *p += 191; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[1] = cells[6] + 335;
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    cells[2] = cells[7] + 797;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[5] = cells[5] + 752;
    cells[1] = cells[5] + 137;
    arr[3 % 6] = 214; cells[3] += arr[3 % 6];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    cells[3] = cells[2] + 629;
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 112u; }
    cells[3] = cells[4] + 856;
    arr[6 % 6] = 415; cells[5] += arr[6 % 6];
    sum += f1(&cells[2], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }