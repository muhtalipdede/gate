import http from 'k6/http';
import { sleep } from 'k6';

export const options = {
    vus: 10000,
    iterations: 100000,
    duration: '10s',
};

export default function() {
    http.get('http://localhost:8080/service1/health');
    // http.get('http://localhost:8080/service2/health');

    sleep(1);
}