static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 2u; }
    cells[1] = cells[3] + 692;
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 492u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    arr[0 % 6] = 534; cells[3] += arr[0 % 6];
    arr[1 % 6] = 859; cells[6] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }