static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[6] + 114;
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 697u; }
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 404u; }
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 975u; }
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    arr[3 % 6] = 339; cells[7] += arr[3 % 6];
    arr[1 % 6] = 554; cells[5] += arr[1 % 6];
    cells[5] = cells[3] + 919;
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 720u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }