static int f1(int *p, int *q) { *p += 403; return *q * 2; }
static int f2(int *p, int *q) { *p += 834; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 484; cells[1] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[2 % 6] = 637; cells[2] += arr[2 % 6];
    sum += f1(&cells[3], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    cells[2] = cells[1] + 962;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 702u; }
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    sum += f2(&cells[1], &cells[7]);
    arr[4 % 6] = 411; cells[7] += arr[4 % 6];
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    cells[1] = cells[7] + 74;
    cells[1] = cells[2] + 697;
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 872u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }