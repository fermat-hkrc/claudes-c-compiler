static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[2] = cells[7] + 909;
    arr[2 % 6] = 568; cells[7] += arr[2 % 6];
    arr[4 % 6] = 243; cells[7] += arr[4 % 6];
    arr[1 % 6] = 720; cells[7] += arr[1 % 6];
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    arr[2 % 6] = 310; cells[3] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    arr[1 % 6] = 39; cells[2] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    cells[6] = cells[6] + 537;
    cells[6] = cells[0] + 574;
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    cells[7] = cells[7] + 31;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }