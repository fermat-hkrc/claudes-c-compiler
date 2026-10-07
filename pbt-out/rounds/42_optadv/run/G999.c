static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    arr[7 % 6] = 252; cells[6] += arr[7 % 6];
    cells[5] = cells[4] + 516;
    arr[7 % 6] = 198; cells[4] += arr[7 % 6];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }