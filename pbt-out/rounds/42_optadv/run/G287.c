static int f1(int *p, int *q) { *p += 688; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    cells[6] = cells[7] + 172;
    arr[3 % 6] = 395; cells[2] += arr[3 % 6];
    sum += f1(&cells[6], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[6 % 6] = 339; cells[5] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    arr[4 % 6] = 173; cells[7] += arr[4 % 6];
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    arr[1 % 6] = 809; cells[5] += arr[1 % 6];
    cells[6] = cells[7] + 774;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }