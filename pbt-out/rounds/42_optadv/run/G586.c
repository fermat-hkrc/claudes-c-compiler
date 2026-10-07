static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[7 % 6] = 530; cells[4] += arr[7 % 6];
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    arr[7 % 6] = 744; cells[2] += arr[7 % 6];
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    cells[5] = cells[4] + 938;
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    arr[0 % 6] = 385; cells[5] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }