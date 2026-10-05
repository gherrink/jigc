/* spawn shim: stands in for `git` (via PATH) or for the jigc test binary path; runs the real program,
 * measures its wall time, appends one line "<name> <depth> <seconds> <first-args>" to $SPAWN_LOG.
 * depth 0 = spawned by the test itself, 1 = spawned from inside a shimmed jigc.
 * Reads: $SHIM_REAL_GIT / $SHIM_REAL_JIGC (the real program), $SPAWN_LOG, $SHIM_DEPTH. Prints: nothing of its
 * own; an argument that is an absolute path or longer than 40 characters is logged as "_".
 * Build it OUTSIDE the repository, into the work directory spawn_probe.sh reads:
 *   cc -O2 -o <work-dir>/shim/shim shim.c && cp <work-dir>/shim/shim <work-dir>/shim/git */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <time.h>
#include <sys/wait.h>
#include <libgen.h>
static double now(void){struct timespec t;clock_gettime(CLOCK_MONOTONIC,&t);return t.tv_sec+t.tv_nsec/1e9;}
int main(int argc,char**argv){
    char *self=basename(argv[0]);
    int is_git=strcmp(self,"git")==0;
    const char *real=getenv(is_git?"SHIM_REAL_GIT":"SHIM_REAL_JIGC");
    const char *log=getenv("SPAWN_LOG");
    const char *d=getenv("SHIM_DEPTH");
    int depth=d?atoi(d):0;
    if(!real){fprintf(stderr,"shim: no real program for %s\n",self);return 127;}
    double t0=now();
    pid_t pid=fork();
    if(pid==0){ if(!is_git) setenv("SHIM_DEPTH","1",1); execv(real,argv); _exit(127); }
    int st=0; waitpid(pid,&st,0);
    double t1=now();
    if(log){ char buf[512]; int n=snprintf(buf,sizeof buf,"%s %d %.4f",is_git?"git":"jigc",depth,t1-t0);
        for(int i=1;i<argc&&i<5&&n<400;i++){ if(argv[i][0]=='/'||strlen(argv[i])>40) n+=snprintf(buf+n,sizeof buf-n," _"); else n+=snprintf(buf+n,sizeof buf-n," %s",argv[i]); }
        buf[n++]='\n'; int fd=open(log,O_WRONLY|O_APPEND|O_CREAT,0644); if(fd>=0){ write(fd,buf,n); close(fd);} }
    if(WIFSIGNALED(st)) return 128+WTERMSIG(st);
    return WEXITSTATUS(st);
}
