static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 247; cells[7] += arr[0 % 6];
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[6 % 6] = 977; cells[6] += arr[6 % 6];
    arr[7 % 6] = 861; cells[0] += arr[7 % 6];
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    arr[0 % 6] = 2; cells[4] += arr[0 % 6];
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }